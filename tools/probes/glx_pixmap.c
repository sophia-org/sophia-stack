#include <GL/gl.h>
#include <GL/glx.h>
#include <GL/glxext.h>
#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int x_error;

static int record_error(Display *display, XErrorEvent *event) {
    (void)display;
    fprintf(stderr, "glx_pixmap x_error=%u major=%u minor=%u\n",
            event->error_code, event->request_code, event->minor_code);
    x_error = 1;
    return 0;
}

static void require(int condition, const char *stage) {
    if (!condition) {
        fprintf(stderr, "glx_pixmap failed=%s\n", stage);
        exit(1);
    }
}

static void check_pixmap(Display *display, GLXFBConfig config, int depth,
                         int glx_target, GLenum gl_target) {
    Pixmap pixmap = XCreatePixmap(display, DefaultRootWindow(display), 3, 1, depth);
    GC gc = XCreateGC(display, pixmap, 0, NULL);
    const int texture_attributes[] = {
        GLX_TEXTURE_TARGET_EXT, glx_target,
        GLX_TEXTURE_FORMAT_EXT, GLX_TEXTURE_FORMAT_RGBA_EXT, None
    };
    GLXPixmap drawable = glXCreatePixmap(display, config, pixmap, texture_attributes);
    XSync(display, False);
    require(drawable != None && !x_error, "create_pixmap");
    unsigned int queried = 0;
    glXQueryDrawable(display, drawable, GLX_TEXTURE_TARGET_EXT, &queried);
    require(queried == (unsigned int)glx_target, "query_target");
    glXQueryDrawable(display, drawable, GLX_TEXTURE_FORMAT_EXT, &queried);
    require(queried == GLX_TEXTURE_FORMAT_RGBA_EXT, "query_format");
    PFNGLXBINDTEXIMAGEEXTPROC bind = (PFNGLXBINDTEXIMAGEEXTPROC)glXGetProcAddressARB((const GLubyte *)"glXBindTexImageEXT");
    PFNGLXRELEASETEXIMAGEEXTPROC release = (PFNGLXRELEASETEXIMAGEEXTPROC)glXGetProcAddressARB((const GLubyte *)"glXReleaseTexImageEXT");
    require(bind && release, "binding_entry_points");
    GLuint texture;
    glGenTextures(1, &texture);
    glBindTexture(gl_target, texture);
    const unsigned long colors[] = {0xff654321, 0xffabcdef};
    const unsigned char expected[][12] = {
        {0x65, 0x43, 0x21, 0xff, 0x65, 0x43, 0x21, 0xff, 0x65, 0x43, 0x21, 0xff},
        {0x65, 0x43, 0x21, 0xff, 0xab, 0xcd, 0xef, 0xff, 0x65, 0x43, 0x21, 0xff}
    };
    for (unsigned int pass = 0; pass < 2; ++pass) {
        XSetForeground(display, gc, colors[pass]);
        XFillRectangle(display, pixmap, gc, pass ? 1 : 0, 0, pass ? 1 : 3, 1);
        XSync(display, False);
        bind(display, drawable, GLX_FRONT_LEFT_EXT, NULL);
        unsigned char pixels[12] = {0};
        glGetTexImage(gl_target, 0, GL_RGBA, GL_UNSIGNED_BYTE, pixels);
        glFinish();
        require(glGetError() == GL_NO_ERROR && !x_error, "readback");
        if (memcmp(pixels, expected[pass], sizeof pixels) != 0) {
            fprintf(stderr, "glx_pixmap mismatch_pass=%u pixels=%02x%02x%02x%02x_%02x%02x%02x%02x\n",
                    pass, pixels[0], pixels[1], pixels[2], pixels[3], pixels[4], pixels[5], pixels[6], pixels[7]);
            exit(1);
        }
        release(display, drawable, GLX_FRONT_LEFT_EXT);
    }
    XFreeGC(display, gc);
    XFreePixmap(display, pixmap);
    XSync(display, False);
    bind(display, drawable, GLX_FRONT_LEFT_EXT, NULL);
    unsigned char retained[12] = {0};
    glGetTexImage(gl_target, 0, GL_RGBA, GL_UNSIGNED_BYTE, retained);
    require(glGetError() == GL_NO_ERROR && !x_error, "retained_readback");
    require(memcmp(retained, expected[1], sizeof retained) == 0, "retained_after_free");
    release(display, drawable, GLX_FRONT_LEFT_EXT);
    glDeleteTextures(1, &texture);
    glXDestroyPixmap(display, drawable);
    XSync(display, False);
    require(!x_error, "pixmap_cleanup");
    printf("glx_pixmap depth=%d target=%d initial=exact dirty=exact retained=exact\n",
           depth, glx_target);
}

int main(int argc, char **argv) {
    const int check_1d = argc == 2 && strcmp(argv[1], "--texture-1d") == 0;
    require(argc == 1 || check_1d, "arguments");
    Display *display = XOpenDisplay(NULL);
    require(display != NULL, "connect");
    XSetErrorHandler(record_error);
    const char *extensions = glXQueryExtensionsString(display, DefaultScreen(display));
    require(extensions && strstr(extensions, "GLX_EXT_texture_from_pixmap"), "advertisement");
    const int attributes[] = {
        GLX_RENDER_TYPE, GLX_RGBA_BIT,
        GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT | GLX_PBUFFER_BIT | GLX_PIXMAP_BIT,
        GLX_X_RENDERABLE, True,
        GLX_RED_SIZE, 8, GLX_GREEN_SIZE, 8, GLX_BLUE_SIZE, 8, GLX_ALPHA_SIZE, 8,
        GLX_DEPTH_SIZE, 24, GLX_STENCIL_SIZE, 8, GLX_DOUBLEBUFFER, True,
        GLX_BIND_TO_TEXTURE_RGBA_EXT, True, None
    };
    int count = 0;
    GLXFBConfig *configs = glXChooseFBConfig(display, DefaultScreen(display), attributes, &count);
    require(configs && count > 0, "matching_config");
    GLXContext context = glXCreateNewContext(display, configs[0], GLX_RGBA_TYPE, NULL, True);
    XSync(display, False);
    require(context != NULL, "create_context");
    require(glXIsDirect(display, context), "direct_context");
    const int pbuffer_attributes[] = {GLX_PBUFFER_WIDTH, 2, GLX_PBUFFER_HEIGHT, 1, None};
    GLXPbuffer pbuffer = glXCreatePbuffer(display, configs[0], pbuffer_attributes);
    require(pbuffer != None && glXMakeContextCurrent(display, pbuffer, pbuffer, context), "current");
    const int targets[] = {GLX_TEXTURE_1D_EXT, GLX_TEXTURE_2D_EXT, GLX_TEXTURE_RECTANGLE_EXT};
    const GLenum gl_targets[] = {GL_TEXTURE_1D, GL_TEXTURE_2D, GL_TEXTURE_RECTANGLE};
    for (int depth = 24; depth <= 32; depth += 8) {
        const int pixmap_attributes[] = {
            GLX_RENDER_TYPE, GLX_RGBA_BIT, GLX_DRAWABLE_TYPE, GLX_PIXMAP_BIT,
            GLX_RED_SIZE, 8, GLX_GREEN_SIZE, 8, GLX_BLUE_SIZE, 8,
            GLX_BIND_TO_TEXTURE_RGBA_EXT, True, None
        };
        int pixmap_count = 0;
        GLXFBConfig *pixmap_configs = glXChooseFBConfig(display, DefaultScreen(display),
                                                       pixmap_attributes, &pixmap_count);
        GLXFBConfig selected = NULL;
        for (int i = 0; i < pixmap_count; ++i) {
            XVisualInfo *visual = glXGetVisualFromFBConfig(display, pixmap_configs[i]);
            if (visual && visual->depth == depth) selected = pixmap_configs[i];
            if (visual) XFree(visual);
            if (selected) break;
        }
        require(selected != NULL, "pixmap_depth_config");
        for (unsigned int i = check_1d ? 0 : 1; i < (check_1d ? 1 : sizeof targets / sizeof targets[0]); ++i)
            check_pixmap(display, selected, depth, targets[i], gl_targets[i]);
        XFree(pixmap_configs);
    }
    glXMakeContextCurrent(display, None, None, NULL);
    glXDestroyPbuffer(display, pbuffer);
    glXDestroyContext(display, context);
    XSync(display, False);
    require(!x_error, "cleanup");
    XFree(configs);
    XCloseDisplay(display);
    puts("glx_pixmap result=pass initial=exact dirty=exact retained=exact");
    return 0;
}
