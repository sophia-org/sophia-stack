// This executable observes its own Qt/XCB calls and reads only its own windows.
#include <QApplication>
#include <QKeyEvent>
#include <QMainWindow>
#include <QMenu>
#include <QMenuBar>
#include <QMouseEvent>
#include <QPlainTextEdit>
#include <QTimer>
#include <QtGui/qguiapplication_platform.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#undef KeyPress
#include <xcb/xinput.h>
#include <dlfcn.h>
#include <cstdio>
#include <cstdlib>
#include <map>
#include <string>

static const char *stage = "startup";
static bool failed;
static int selections;
static std::map<std::string, int> grabs;
static std::map<xcb_window_t, unsigned> maps;
static std::map<unsigned, xcb_window_t> grab_windows;

// Forward the exact request/reply. Observation must not issue an extra request
// between Qt's MapWindow and GrabDevice: that would mask the ordering defect.
template<typename Function> static Function next(const char *name) {
    auto function = reinterpret_cast<Function>(dlsym(RTLD_NEXT, name));
    if (!function) { std::fprintf(stderr, "qt_popup missing_symbol=%s\n", name); std::abort(); }
    return function;
}
extern "C" xcb_void_cookie_t xcb_map_window(xcb_connection_t *c, xcb_window_t window) {
    static auto call = next<decltype(&xcb_map_window)>("xcb_map_window");
    auto cookie = call(c, window);
    maps[window] = cookie.sequence;
    return cookie;
}
extern "C" xcb_input_xi_grab_device_cookie_t xcb_input_xi_grab_device(
        xcb_connection_t *c, xcb_window_t window, xcb_timestamp_t time, xcb_cursor_t cursor,
        xcb_input_device_id_t device, uint8_t mode, uint8_t paired_mode,
        uint8_t owner_events, uint16_t mask_len, const uint32_t *mask) {
    static auto call = next<decltype(&xcb_input_xi_grab_device)>("xcb_input_xi_grab_device");
    auto cookie = call(c, window, time, cursor, device, mode, paired_mode, owner_events, mask_len, mask);
    grab_windows[cookie.sequence] = window;
    return cookie;
}
extern "C" xcb_grab_pointer_cookie_t xcb_grab_pointer(
        xcb_connection_t *c, uint8_t owner_events, xcb_window_t window, uint16_t event_mask,
        uint8_t pointer_mode, uint8_t keyboard_mode, xcb_window_t confine,
        xcb_cursor_t cursor, xcb_timestamp_t time) {
    static auto call = next<decltype(&xcb_grab_pointer)>("xcb_grab_pointer");
    auto cookie = call(c, owner_events, window, event_mask, pointer_mode, keyboard_mode, confine, cursor, time);
    grab_windows[cookie.sequence] = window;
    return cookie;
}
static void observed_grab(const char *api, unsigned sequence, int status) {
    ++grabs[stage];
    auto found = grab_windows.find(sequence);
    xcb_window_t window = found == grab_windows.end() ? 0 : found->second;
    bool map_before = maps.count(window) && maps[window] < sequence;
    failed |= status != 0 || !map_before;
    std::printf("qt_popup grab stage=%s api=%s sequence=%u xid=%u mapped_before=%d status=%d\n",
        stage, api, sequence, window, map_before, status);
    std::fflush(stdout);
}
extern "C" xcb_grab_pointer_reply_t *xcb_grab_pointer_reply(
        xcb_connection_t *c, xcb_grab_pointer_cookie_t cookie, xcb_generic_error_t **error) {
    static auto call = next<decltype(&xcb_grab_pointer_reply)>("xcb_grab_pointer_reply");
    auto *reply = call(c, cookie, error);
    observed_grab("core", cookie.sequence, reply ? reply->status : -1);
    return reply;
}
extern "C" xcb_input_xi_grab_device_reply_t *xcb_input_xi_grab_device_reply(
        xcb_connection_t *c, xcb_input_xi_grab_device_cookie_t cookie, xcb_generic_error_t **error) {
    static auto call = next<decltype(&xcb_input_xi_grab_device_reply)>("xcb_input_xi_grab_device_reply");
    auto *reply = call(c, cookie, error);
    observed_grab("xi2", cookie.sequence, reply ? reply->status : -1);
    return reply;
}
static void click(QWidget *widget, QPoint position) {
    QMouseEvent press(QEvent::MouseButtonPress, QPointF(position),
        QPointF(widget->mapToGlobal(position)), Qt::LeftButton, Qt::LeftButton, Qt::NoModifier);
    QApplication::sendEvent(widget, &press);
    QMouseEvent release(QEvent::MouseButtonRelease, QPointF(position),
        QPointF(widget->mapToGlobal(position)), Qt::LeftButton, Qt::NoButton, Qt::NoModifier);
    QApplication::sendEvent(widget, &release);
}
static void capture(QMenu &menu, const char *name, Window expected_owner, const std::string &directory) {
    Display *display = XOpenDisplay(nullptr);
    XWindowAttributes attributes{};
    Window owner = 0;
    const Window window = menu.winId();
    bool visible = display && XGetWindowAttributes(display, window, &attributes)
        && attributes.map_state == IsViewable;
    bool owned = display && XGetTransientForHint(display, window, &owner) && owner == expected_owner;
    XImage *pixels = visible ? XGetImage(display, window, 0, 0, attributes.width,
        attributes.height, AllPlanes, ZPixmap) : nullptr;
    int light = 0, dark = 0;
    FILE *file = pixels ? std::fopen((directory + "/" + name + ".ppm").c_str(), "wb") : nullptr;
    if (file) std::fprintf(file, "P6\n%d %d\n255\n", attributes.width, attributes.height);
    if (pixels) {
        for (int y = 0; y < attributes.height; ++y) {
            for (int x = 0; x < attributes.width; ++x) {
                unsigned long value = XGetPixel(pixels, x, y);
                unsigned char rgb[] = {static_cast<unsigned char>(value >> 16),
                    static_cast<unsigned char>(value >> 8), static_cast<unsigned char>(value)};
                if (file) std::fwrite(rgb, 1, 3, file);
                int sum = rgb[0] + rgb[1] + rgb[2];
                light += sum > 660;
                dark += sum < 300;
            }
        }
        XDestroyImage(pixels);
    }
    bool saved = file && std::fclose(file) == 0;
    bool content = light > attributes.width * attributes.height / 3 && dark > 10;
    bool map_seen = maps.count(window) != 0;
    bool passed = visible && owned && content && saved && map_seen;
    failed |= !passed;
    std::printf("qt_popup capture=%s xid=%lu owner=%lu expected_owner=%lu mapped=%d map_observed=%d content=%s saved=%d result=%s\n",
        name, window, owner, expected_owner, visible, map_seen, content ? "pass" : "fail", saved, passed ? "pass" : "fail");
    std::fflush(stdout);
    if (display) XCloseDisplay(display);
}
int main(int argc, char **argv) {
    QApplication app(argc, argv);
    app.setStyle("Fusion");
    app.setStyleSheet("QMenu { color: #000; background: #fff; font: 14px monospace; }"
        "QMenu::item:selected { color: #fff; background: #345678; }");
    QMainWindow window;
    window.setCentralWidget(new QPlainTextEdit("Synthetic Qt popup probe", &window));
    QMenu *menu = window.menuBar()->addMenu("File");
    QAction *select = menu->addAction("Select marker");
    menu->addAction("Second marker");
    QMenu *nested = menu->addMenu("Nested menu");
    QAction *nested_select = nested->addAction("Select nested marker");
    nested->addAction("Second nested marker");
    QObject::connect(select, &QAction::triggered, [&] { ++selections; });
    QObject::connect(nested_select, &QAction::triggered, [&] { ++selections; });
    window.resize(640, 480);
    window.show();
    const std::string directory = QApplication::applicationDirPath().toStdString();
    auto open = [&] {
        click(window.menuBar(), window.menuBar()->actionGeometry(menu->menuAction()).center());
    };
    QTimer::singleShot(1200, [&] { stage = "first"; open(); });
    QTimer::singleShot(2400, [&] {
        capture(*menu, "first", window.winId(), directory);
        failed |= grabs["first"] == 0;
        click(menu, menu->actionGeometry(select).center());
        bool ok = selections == 1 && !menu->isVisible();
        failed |= !ok;
        std::printf("qt_popup selection=first result=%s\n", ok ? "pass" : "fail");
    });
    QTimer::singleShot(3200, [&] { stage = "reopen"; open(); });
    QTimer::singleShot(4300, [&] {
        capture(*menu, "reopen", window.winId(), directory);
        failed |= grabs["reopen"] == 0;
        stage = "nested";
        menu->setActiveAction(nested->menuAction());
        QKeyEvent right(QEvent::KeyPress, Qt::Key_Right, Qt::NoModifier);
        QApplication::sendEvent(menu, &right);
    });
    QTimer::singleShot(5400, [&] {
        // Nested ownership is a real chain: submenu -> menu -> main window.
        capture(*nested, "nested", menu->winId(), directory);
        click(nested, nested->actionGeometry(nested_select).center());
        bool ok = selections == 2 && !menu->isVisible() && !nested->isVisible();
        failed |= !ok;
        std::printf("qt_popup selection=nested result=%s\n", ok ? "pass" : "fail");
    });
    QTimer::singleShot(6200, [&] { stage = "dismiss"; open(); });
    xcb_window_t raw_window = 0;
    bool raw_granted = false;
    auto *native = app.nativeInterface<QNativeInterface::QX11Application>();
    if (!native) return 2;
    xcb_connection_t *connection = native->connection();
    QTimer::singleShot(7300, [&] {
        capture(*menu, "dismiss", window.winId(), directory);
        failed |= grabs["dismiss"] == 0;
        QKeyEvent escape(QEvent::KeyPress, Qt::Key_Escape, Qt::NoModifier);
        QApplication::sendEvent(menu, &escape);
        bool ok = !menu->isVisible() && !nested->isVisible() && selections == 2;
        failed |= !ok;
        std::printf("qt_popup dismiss=escape result=%s\n", ok ? "pass" : "fail");
        std::fflush(stdout);
    });
    QTimer::singleShot(8100, [&] {
        stage = "draw-after-grab";
        const auto *screen = xcb_setup_roots_iterator(xcb_get_setup(connection)).data;
        raw_window = xcb_generate_id(connection);
        const uint32_t override_redirect = 1;
        xcb_create_window(connection, XCB_COPY_FROM_PARENT, raw_window, screen->root,
            30, 80, 96, 64, 0, XCB_WINDOW_CLASS_INPUT_OUTPUT, screen->root_visual,
            XCB_CW_OVERRIDE_REDIRECT, &override_redirect);
        const xcb_window_t parent = window.winId();
        xcb_change_property(connection, XCB_PROP_MODE_REPLACE, raw_window,
            XCB_ATOM_WM_TRANSIENT_FOR, XCB_ATOM_WINDOW, 32, 1, &parent);
        // No drawing, round trip, logging or settling delay between map and grab.
        xcb_map_window(connection, raw_window);
        auto cookie = xcb_grab_pointer(connection, 0, raw_window,
            XCB_EVENT_MASK_BUTTON_PRESS | XCB_EVENT_MASK_BUTTON_RELEASE,
            XCB_GRAB_MODE_ASYNC, XCB_GRAB_MODE_ASYNC, XCB_NONE, XCB_NONE, XCB_CURRENT_TIME);
        xcb_generic_error_t *error = nullptr;
        auto *reply = xcb_grab_pointer_reply(connection, cookie, &error);
        raw_granted = reply && !error && reply->status == XCB_GRAB_STATUS_SUCCESS;
        std::free(reply);
        std::free(error);
        failed |= !raw_granted;
        if (raw_granted) {
            const xcb_gcontext_t gc = xcb_generate_id(connection);
            const uint32_t marker = 0x216543;
            xcb_create_gc(connection, gc, raw_window, XCB_GC_FOREGROUND, &marker);
            const xcb_rectangle_t rectangle{0, 0, 96, 64};
            xcb_poly_fill_rectangle(connection, raw_window, gc, 1, &rectangle);
            xcb_free_gc(connection, gc);
            xcb_flush(connection);
        }
        std::printf("qt_popup raw_grab=%s drew_after_success=%d\n", raw_granted ? "pass" : "fail", raw_granted);
        std::fflush(stdout);
    });
    QTimer::singleShot(9300, [&] {
        Display *display = XOpenDisplay(nullptr);
        XWindowAttributes attributes{};
        Window owner = 0;
        bool mapped = display && XGetWindowAttributes(display, raw_window, &attributes)
            && attributes.map_state == IsViewable;
        bool owned = display && XGetTransientForHint(display, raw_window, &owner)
            && owner == window.winId();
        XImage *pixels = mapped && raw_granted ? XGetImage(display, raw_window, 0, 0,
            96, 64, AllPlanes, ZPixmap) : nullptr;
        bool exact = pixels;
        if (pixels) {
            for (int y = 0; y < 64; ++y)
                for (int x = 0; x < 96; ++x)
                    exact &= (XGetPixel(pixels, x, y) & 0xffffff) == 0x216543;
            XDestroyImage(pixels);
        }
        failed |= !mapped || !owned || !exact;
        std::printf("qt_popup raw_pixels=%s mapped=%d owned=%d\n", exact ? "pass" : "fail", mapped, owned);
        if (display) XCloseDisplay(display);
        if (raw_granted) xcb_ungrab_pointer(connection, XCB_CURRENT_TIME);
        xcb_destroy_window(connection, raw_window);
        xcb_flush(connection);
        std::printf("qt_popup complete result=%s selections=%d\n", failed ? "fail" : "pass", selections);
        std::fflush(stdout);
        app.exit(failed ? 1 : 0);
    });
    return app.exec();
}
