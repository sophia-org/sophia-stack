/* Synthetic GTK3 pixels only: no user input, files, clipboard or desktop capture. */
#include <gtk/gtk.h>
#include <gdk/gdkx.h>
#include <stdio.h>

static GtkWidget *main_window, *dialog, *menu;
static char *output_dir;
static gboolean failed;

static void check_focus_viewable(void) {
    Display *display = gdk_x11_display_get_xdisplay(gdk_display_get_default());
    Window focus;
    int revert_to;
    XWindowAttributes attributes;
    XGetInputFocus(display, &focus, &revert_to);
    gboolean eligible = focus == None || focus == PointerRoot ||
        (XGetWindowAttributes(display, focus, &attributes) &&
         attributes.map_state == IsViewable);
    printf("gtk_redraw focus_after_dialog_unmap=%s\n", eligible ? "pass" : "fail");
    fflush(stdout);
    failed |= !eligible;
}

/* Each controlled white widget must contain both its background and text.
 * Check the dialog halves separately so a surviving Close button cannot hide
 * the original missing-message failure. The menu is checked row by row. */
static gboolean has_text_and_background(GdkPixbuf *pixels, int bands) {
    int width = gdk_pixbuf_get_width(pixels);
    int height = gdk_pixbuf_get_height(pixels);
    int stride = gdk_pixbuf_get_rowstride(pixels);
    int channels = gdk_pixbuf_get_n_channels(pixels);
    const guchar *data = gdk_pixbuf_read_pixels(pixels);
    for (int band = 0; band < bands; ++band) {
        int light = 0, dark = 0, total = 0;
        for (int y = band * height / bands + 3;
             y < (band + 1) * height / bands - 3; ++y) {
            for (int x = 8; x < width - 8; ++x) {
                const guchar *p = data + y * stride + x * channels;
                int sum = p[0] + p[1] + p[2];
                light += sum > 660;
                dark += sum < 300;
                ++total;
            }
        }
        if (total == 0 || light < total / 2 || dark < 5)
            return FALSE;
    }
    return TRUE;
}

static void capture(GtkWidget *widget, const char *name, int bands) {
    GdkWindow *window = gtk_widget_get_window(widget);
    if (!window) { failed = TRUE; return; }
    int width = gdk_window_get_width(window);
    int height = gdk_window_get_height(window);
    GdkPixbuf *pixels = gdk_pixbuf_get_from_window(window, 0, 0, width, height);
    gboolean content_ok = pixels && (!bands || has_text_and_background(pixels, bands));
    gboolean saved = FALSE;
    if (pixels) {
        char *filename = g_strdup_printf("%s/%s.png", output_dir, name);
        saved = gdk_pixbuf_save(pixels, filename, "png", NULL, NULL);
        g_free(filename);
        g_object_unref(pixels);
    }
    printf("gtk_redraw capture=%s size=%dx%d content=%s saved=%d\n",
           name, width, height, content_ok ? "pass" : "fail", saved);
    fflush(stdout);
    failed |= !content_ok || !saved;
}

static gboolean step(gpointer unused) {
    static int phase;
    (void)unused;
    switch (phase++) {
    case 0:
        capture(main_window, "main", 0);
        dialog = gtk_message_dialog_new(GTK_WINDOW(main_window), 0,
            GTK_MESSAGE_INFO, GTK_BUTTONS_CLOSE, "A complete GTK dialog");
        gtk_widget_show_all(dialog);
        break;
    case 1:
        capture(dialog, "dialog", 2);
        gtk_widget_hide(dialog);
        menu = gtk_menu_new();
        for (int i = 0; i < 8; ++i) {
            char *label = g_strdup_printf("Menu item %d", i);
            gtk_menu_shell_append(GTK_MENU_SHELL(menu), gtk_menu_item_new_with_label(label));
            g_free(label);
        }
        gtk_widget_show_all(menu);
        gtk_menu_popup_at_widget(GTK_MENU(menu), main_window,
            GDK_GRAVITY_NORTH_WEST, GDK_GRAVITY_NORTH_WEST, NULL);
        break;
    case 2:
        check_focus_viewable();
        capture(gtk_widget_get_toplevel(menu), "menu", 8);
        gtk_menu_popdown(GTK_MENU(menu));
        gtk_widget_show_all(dialog);
        break;
    case 3:
        capture(dialog, "dialog-remap", 2);
        gtk_widget_queue_draw(dialog);
        break;
    case 4:
        capture(dialog, "dialog-redraw", 2);
        gtk_widget_destroy(dialog);
        gtk_widget_destroy(menu);
        gtk_widget_destroy(main_window);
        gtk_main_quit();
        return G_SOURCE_REMOVE;
    }
    return G_SOURCE_CONTINUE;
}

int main(int argc, char **argv) {
    output_dir = g_path_get_dirname(argv[0]);
    gtk_init(&argc, &argv);
    GtkCssProvider *css = gtk_css_provider_new();
    gtk_css_provider_load_from_data(css,
        "* { color: #000; background-color: #fff; background-image: none;"
        " text-shadow: none; box-shadow: none; font-family: monospace;"
        " font-size: 14px; } menu { padding: 0; }", -1, NULL);
    gtk_style_context_add_provider_for_screen(gdk_screen_get_default(),
        GTK_STYLE_PROVIDER(css), GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
    g_object_set(gtk_settings_get_default(), "gtk-enable-animations", FALSE, NULL);
    g_object_unref(css);
    main_window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_default_size(GTK_WINDOW(main_window), 640, 400);
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_container_add(GTK_CONTAINER(main_window), box);
    gtk_box_pack_start(GTK_BOX(box), gtk_label_new("GTK redraw probe"), FALSE, FALSE, 0);
    gtk_box_pack_start(GTK_BOX(box), gtk_button_new_with_label("Sidebar row"), TRUE, TRUE, 0);
    gtk_widget_show_all(main_window);
    /* Settling time belongs to this optional real-client probe, never to
     * production rendering. Wire regressions cover unpaced request bursts. */
    g_timeout_add(1500, step, NULL);
    gtk_main();
    g_free(output_dir);
    return failed ? 1 : 0;
}
