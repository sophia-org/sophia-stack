// Receives real frontend XI events. No synthetic Qt or X input is injected here.
#include <QAbstractNativeEventFilter>
#include <QApplication>
#include <QMouseEvent>
#include <QInputDevice>
#include <QPlainTextEdit>
#include <QScrollBar>
#include <QTimer>
#include <QtGui/qguiapplication_platform.h>
#include <QWheelEvent>
#include <xcb/xinput.h>
#include <cstdio>
#include <vector>

struct NativeObserver : QAbstractNativeEventFilter {
    unsigned source_motion = 0, master_motion = 0;
    unsigned core_buttons = 0;
    unsigned source_press = 0, master_press = 0;
    unsigned source_release = 0, master_release = 0;
    bool bad_source = false;
    uint8_t opcode = 0;
    bool nativeEventFilter(const QByteArray &type, void *message, qintptr *) override {
        if (type != "xcb_generic_event_t") return false;
        auto *generic = static_cast<xcb_generic_event_t *>(message);
        if ((generic->response_type & 0x7f) == XCB_BUTTON_PRESS || (generic->response_type & 0x7f) == XCB_BUTTON_RELEASE) {
            auto *button = static_cast<xcb_button_press_event_t *>(message);
            if (button->detail == 1) {
                ++core_buttons;
                std::printf("qt_wheel core type=%u detail=%u time=%u\n", generic->response_type & 0x7f, button->detail, button->time);
                std::fflush(stdout);
            }
        }
        if ((generic->response_type & 0x7f) != XCB_GE_GENERIC) return false;
        auto *event = static_cast<xcb_input_button_press_event_t *>(message);
        if (event->extension != opcode) return false;
        if (event->event_type != XCB_INPUT_MOTION && event->event_type != XCB_INPUT_BUTTON_PRESS
            && event->event_type != XCB_INPUT_BUTTON_RELEASE) return false;
        // Wheel compatibility buttons accompany the smooth valuator event.
        if (event->event_type != XCB_INPUT_MOTION && event->detail != 1) return false;
        bad_source |= event->sourceid != 128 || (event->deviceid != 2 && event->deviceid != 128);
        std::printf("qt_wheel native type=%u device=%u source=%u detail=%u flags=%u valuators_words=%u\n",
            event->event_type, event->deviceid, event->sourceid, event->detail, event->flags, event->valuators_len);
        std::fflush(stdout);
        bool source = event->deviceid == 128;
        if (event->event_type == XCB_INPUT_MOTION) {
            if (event->valuators_len != 0) ++(source ? source_motion : master_motion);
        }
        else if (event->event_type == XCB_INPUT_BUTTON_PRESS) ++(source ? source_press : master_press);
        else ++(source ? source_release : master_release);
        return false;
    }
};

struct WidgetObserver : QObject {
    QPlainTextEdit *editor = nullptr;
    NativeObserver *native = nullptr;
    std::vector<int> angles;
    unsigned presses = 0, releases = 0;
    bool movement_ok = true;
    bool positioned = false;
    bool eventFilter(QObject *target, QEvent *event) override {
        if (target != editor->viewport()) return false;
        if (event->type() == QEvent::MouseMove && !positioned) {
            positioned = true;
            QTimer::singleShot(0, this, [] {
                std::printf("qt_wheel positioned=1\n");
                std::fflush(stdout);
            });
        }
        if (event->type() == QEvent::Wheel) {
            int angle = static_cast<QWheelEvent *>(event)->angleDelta().y();
            angles.push_back(angle);
            int before = editor->verticalScrollBar()->value();
            const auto count = angles.size();
            QTimer::singleShot(0, this, [this, angle, before, count] {
                int after = editor->verticalScrollBar()->value();
                movement_ok &= angle < 0 ? after > before : after < before;
                std::printf("qt_wheel wheel=%zu angle=%d before=%d after=%d\n", count, angle, before, after);
                std::fflush(stdout);
            });
        } else if (event->type() == QEvent::MouseButtonPress || event->type() == QEvent::MouseButtonDblClick) {
            ++presses;
            std::printf("qt_wheel press=%u type=%d spontaneous=%d device=%lld time=%lu\n", presses,
                int(event->type()), event->spontaneous(),
                static_cast<long long>(static_cast<QMouseEvent *>(event)->pointingDevice()->systemId()),
                static_cast<unsigned long>(static_cast<QMouseEvent *>(event)->timestamp()));
            std::fflush(stdout);
        } else if (event->type() == QEvent::MouseButtonRelease) {
            ++releases;
            // Let any duplicate or emulated events already on the socket arrive.
            QTimer::singleShot(150, this, [this] {
                bool passed = angles == std::vector<int>({-120, 120, 120, -120})
                    && movement_ok && presses == 1 && releases == 1 && !native->bad_source
                    && native->core_buttons == 0
                    && native->source_motion == 4 && native->master_motion == 4
                    && native->source_press == 1 && native->master_press == 1
                    && native->source_release == 1 && native->master_release == 1;
                std::printf("qt_wheel complete result=%s wheels=%zu presses=%u releases=%u source_motion=%u master_motion=%u source_press=%u master_press=%u source_release=%u master_release=%u core_buttons=%u bad_source=%d\n",
                    passed ? "pass" : "fail", angles.size(), presses, releases,
                    native->source_motion, native->master_motion, native->source_press,
                    native->master_press, native->source_release, native->master_release, native->core_buttons, native->bad_source);
                std::fflush(stdout);
                QApplication::exit(passed ? 0 : 1);
            });
        }
        return false;
    }
};

int main(int argc, char **argv) {
    QApplication app(argc, argv);
    // XCB platform initialization enables compression. Disable it afterwards so
    // the native observer sees both selected source and master Motion packets.
    QCoreApplication::setAttribute(Qt::AA_CompressHighFrequencyEvents, false);
    QPlainTextEdit editor;
    QString text;
    for (int row = 0; row < 200; ++row) text += QString("Synthetic line %1\n").arg(row);
    editor.setPlainText(text);
    editor.resize(640, 480);
    editor.setMouseTracking(true);
    editor.viewport()->setMouseTracking(true);
    editor.show();
    NativeObserver native;
    auto *xcb = app.nativeInterface<QNativeInterface::QX11Application>()->connection();
    native.opcode = xcb_get_extension_data(xcb, &xcb_input_id)->major_opcode;
    WidgetObserver widgets;
    widgets.editor = &editor;
    widgets.native = &native;
    app.installNativeEventFilter(&native);
    app.installEventFilter(&widgets);
    QTimer::singleShot(300, [&] {
        for (const auto *device : QInputDevice::devices()) {
            std::printf("qt_wheel device=%lld class=%s name=%s\n",
                static_cast<long long>(device->systemId()), device->metaObject()->className(),
                device->name().toUtf8().constData());
        }
        editor.verticalScrollBar()->setValue(50);
        std::printf("qt_wheel ready xid=%lu qt=%s\n", static_cast<unsigned long>(editor.winId()), qVersion());
        std::fflush(stdout);
    });
    QTimer::singleShot(10000, [&] {
        std::printf("qt_wheel timeout wheels=%zu presses=%u releases=%u source_motion=%u master_motion=%u\n",
            widgets.angles.size(), widgets.presses, widgets.releases, native.source_motion, native.master_motion);
        std::fflush(stdout);
        app.exit(1);
    });
    return app.exec();
}
