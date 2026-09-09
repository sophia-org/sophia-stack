#include <EGL/egl.h>
#include <EGL/eglext.h>
#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>
#include <GL/glx.h>
#include <GL/glxext.h>
#include <X11/Xlib.h>
#include <X11/Xlib-xcb.h>
#include <xcb/dri3.h>
#include <gbm.h>
#include <fcntl.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

static int x_error;

static int record_error(Display *display, XErrorEvent *event) {
    (void)display;
    fprintf(stderr, "egl_pixmap x_error=%u major=%u minor=%u\n",
            event->error_code, event->request_code, event->minor_code);
    x_error = 1;
    return 0;
}

static void require(int condition, const char *stage) {
    if (!condition) {
        fprintf(stderr, "egl_pixmap failed=%s egl_error=0x%x gl_error=0x%x\n",
                stage, eglGetError(), glGetError());
        exit(1);
    }
    printf("egl_pixmap stage=%s result=pass\n", stage);
    fflush(stdout);
}

struct producer {
    int device_fd;
    struct gbm_device *device;
    struct gbm_bo *buffer;
    EGLDisplay display;
    EGLContext context;
    EGLImageKHR image;
    GLuint renderbuffer;
    GLuint framebuffer;
};

static struct producer make_producer(void) {
    struct producer source = {0};
    const char *node = getenv("SOPHIA_PIXMAP_TEST_DEVICE");
    require(node != NULL, "producer_device_selection");
    source.device_fd = open(node, O_RDWR | O_CLOEXEC);
    require(source.device_fd >= 0, "producer_device_open");
    source.device = gbm_create_device(source.device_fd);
    require(source.device != NULL, "producer_gbm_device");
    source.buffer = gbm_bo_create(source.device, 3, 1, GBM_FORMAT_ARGB8888, GBM_BO_USE_RENDERING);
    require(source.buffer != NULL, "producer_buffer");
    uint64_t modifier = gbm_bo_get_modifier(source.buffer);
    int planes = gbm_bo_get_plane_count(source.buffer);
    printf("egl_pixmap producer_format=0x%x modifier=0x%016" PRIx64 " planes=%d stride=%u declared_size=0\n",
           gbm_bo_get_format(source.buffer), modifier, planes, gbm_bo_get_stride(source.buffer));
    require(planes >= 1 && planes <= 4, "producer_plane_count");
    PFNEGLGETPLATFORMDISPLAYEXTPROC platform_display =
        (PFNEGLGETPLATFORMDISPLAYEXTPROC)eglGetProcAddress("eglGetPlatformDisplayEXT");
    require(platform_display != NULL, "producer_platform_entry_point");
    source.display = platform_display(EGL_PLATFORM_GBM_KHR, source.device, NULL);
    require(source.display != EGL_NO_DISPLAY && eglInitialize(source.display, NULL, NULL), "producer_display");
    require(eglBindAPI(EGL_OPENGL_ES_API), "producer_api");
    const EGLint attributes[] = {EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT, EGL_RED_SIZE, 8,
                                EGL_GREEN_SIZE, 8, EGL_BLUE_SIZE, 8, EGL_ALPHA_SIZE, 8, EGL_NONE};
    EGLConfig config = NULL;
    EGLint count = 0;
    require(eglChooseConfig(source.display, attributes, &config, 1, &count) && count, "producer_config");
    const EGLint context_attributes[] = {EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE};
    source.context = eglCreateContext(source.display, config, EGL_NO_CONTEXT, context_attributes);
    require(source.context != EGL_NO_CONTEXT, "producer_context");
    require(eglMakeCurrent(source.display, EGL_NO_SURFACE, EGL_NO_SURFACE, source.context), "producer_current");
    const EGLint fd_keys[] = {EGL_DMA_BUF_PLANE0_FD_EXT, EGL_DMA_BUF_PLANE1_FD_EXT,
                             EGL_DMA_BUF_PLANE2_FD_EXT, EGL_DMA_BUF_PLANE3_FD_EXT};
    const EGLint offset_keys[] = {EGL_DMA_BUF_PLANE0_OFFSET_EXT, EGL_DMA_BUF_PLANE1_OFFSET_EXT,
                                 EGL_DMA_BUF_PLANE2_OFFSET_EXT, EGL_DMA_BUF_PLANE3_OFFSET_EXT};
    const EGLint pitch_keys[] = {EGL_DMA_BUF_PLANE0_PITCH_EXT, EGL_DMA_BUF_PLANE1_PITCH_EXT,
                                EGL_DMA_BUF_PLANE2_PITCH_EXT, EGL_DMA_BUF_PLANE3_PITCH_EXT};
    const EGLint lo_keys[] = {EGL_DMA_BUF_PLANE0_MODIFIER_LO_EXT, EGL_DMA_BUF_PLANE1_MODIFIER_LO_EXT,
                             EGL_DMA_BUF_PLANE2_MODIFIER_LO_EXT, EGL_DMA_BUF_PLANE3_MODIFIER_LO_EXT};
    EGLint image_attributes[48] = {EGL_WIDTH, 3, EGL_HEIGHT, 1,
                                  EGL_LINUX_DRM_FOURCC_EXT, GBM_FORMAT_ARGB8888};
    int plane_fds[4] = {-1, -1, -1, -1};
    size_t index = 6;
    for (int plane = 0; plane < planes; ++plane) {
        plane_fds[plane] = gbm_bo_get_fd_for_plane(source.buffer, plane);
        require(plane_fds[plane] >= 0, "producer_plane_fd");
        image_attributes[index++] = fd_keys[plane];
        image_attributes[index++] = plane_fds[plane];
        image_attributes[index++] = offset_keys[plane];
        image_attributes[index++] = (EGLint)gbm_bo_get_offset(source.buffer, plane);
        image_attributes[index++] = pitch_keys[plane];
        image_attributes[index++] = (EGLint)gbm_bo_get_stride_for_plane(source.buffer, plane);
        image_attributes[index++] = lo_keys[plane];
        image_attributes[index++] = (EGLint)(uint32_t)modifier;
        image_attributes[index++] = lo_keys[plane] + 1;
        image_attributes[index++] = (EGLint)(uint32_t)(modifier >> 32);
    }
    image_attributes[index] = EGL_NONE;
    PFNEGLCREATEIMAGEKHRPROC create_image = (PFNEGLCREATEIMAGEKHRPROC)eglGetProcAddress("eglCreateImageKHR");
    PFNGLEGLIMAGETARGETRENDERBUFFERSTORAGEOESPROC image_storage =
        (PFNGLEGLIMAGETARGETRENDERBUFFERSTORAGEOESPROC)eglGetProcAddress("glEGLImageTargetRenderbufferStorageOES");
    require(create_image && image_storage, "producer_image_entry_points");
    source.image = create_image(source.display, EGL_NO_CONTEXT, EGL_LINUX_DMA_BUF_EXT, NULL, image_attributes);
    for (int plane = 0; plane < planes; ++plane) close(plane_fds[plane]);
    require(source.image != EGL_NO_IMAGE_KHR, "producer_image");
    glGenRenderbuffers(1, &source.renderbuffer);
    glBindRenderbuffer(GL_RENDERBUFFER, source.renderbuffer);
    image_storage(GL_RENDERBUFFER, source.image);
    glGenFramebuffers(1, &source.framebuffer);
    glBindFramebuffer(GL_FRAMEBUFFER, source.framebuffer);
    glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_RENDERBUFFER, source.renderbuffer);
    require(glCheckFramebufferStatus(GL_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE, "producer_framebuffer");
    return source;
}

static void paint_producer(struct producer *source, int dirty) {
    require(eglMakeCurrent(source->display, EGL_NO_SURFACE, EGL_NO_SURFACE, source->context), "producer_paint_current");
    glBindFramebuffer(GL_FRAMEBUFFER, source->framebuffer);
    glViewport(0, 0, 3, 1);
    glDisable(GL_DITHER);
    glEnable(GL_SCISSOR_TEST);
    glScissor(dirty ? 1 : 0, 0, dirty ? 1 : 3, 1);
    glClearColor((dirty ? 0xab : 0x65) / 255.0f, (dirty ? 0xcd : 0x43) / 255.0f,
                 (dirty ? 0xef : 0x21) / 255.0f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
    glFinish();
    require(glGetError() == GL_NO_ERROR, "producer_paint_complete");
    const unsigned char initial[] = {0x65,0x43,0x21,0xff, 0x65,0x43,0x21,0xff, 0x65,0x43,0x21,0xff};
    const unsigned char changed[] = {0x65,0x43,0x21,0xff, 0xab,0xcd,0xef,0xff, 0x65,0x43,0x21,0xff};
    unsigned char pixels[12] = {0};
    glReadPixels(0, 0, 3, 1, GL_RGBA, GL_UNSIGNED_BYTE, pixels);
    require(glGetError() == GL_NO_ERROR && memcmp(pixels, dirty ? changed : initial, sizeof pixels) == 0,
            "producer_pixels_exact");
    require(eglMakeCurrent(source->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT), "producer_paint_release");
}

static Pixmap import_producer(Display *owner, struct producer *source, int wire_implicit) {
    xcb_connection_t *connection = XGetXCBConnection(owner);
    require(connection != NULL, "import_connection");
    xcb_pixmap_t pixmap = xcb_generate_id(connection);
    int fd = gbm_bo_get_fd(source->buffer);
    struct stat original;
    require(fd >= 0 && fstat(fd, &original) == 0, "import_fd_identity");
    uint32_t stride = gbm_bo_get_stride(source->buffer);
    require(fd >= 0 && stride <= UINT16_MAX, "import_descriptor");
    xcb_void_cookie_t cookie = wire_implicit
        ? xcb_dri3_pixmap_from_buffers_checked(connection, pixmap, DefaultRootWindow(owner),
            1, 3, 1, stride, 0, 0, 0, 0, 0, 0, 0, 32, 32,
            gbm_bo_get_modifier(source->buffer), &fd)
        : xcb_dri3_pixmap_from_buffer_checked(connection, pixmap,
            DefaultRootWindow(owner), 0, 3, 1, (uint16_t)stride, 32, 32, fd);
    xcb_generic_error_t *error = xcb_request_check(connection, cookie);
    if (error) fprintf(stderr, "egl_pixmap import_error=%u major=%u minor=%u\n",
                       error->error_code, error->major_code, error->minor_code);
    require(error == NULL, wire_implicit ? "modifier_dri3_import" : "legacy_dri3_import");
    free(error);
    error = NULL;
    xcb_dri3_buffers_from_pixmap_reply_t *reply = xcb_dri3_buffers_from_pixmap_reply(connection,
        xcb_dri3_buffers_from_pixmap(connection, pixmap), &error);
    require(reply != NULL && error == NULL && reply->nfd == 1, "import_export_roundtrip");
    require(reply->modifier == UINT64_C(0x00ffffffffffffff), "returned_wire_implicit_modifier");
    int *reply_fds = xcb_dri3_buffers_from_pixmap_reply_fds(connection, reply);
    struct stat returned;
    require(fstat(reply_fds[0], &returned) == 0, "returned_fd_identity");
    printf("egl_pixmap imported_xid=0x%x returned_width=%u returned_height=%u returned_modifier=0x%016" PRIx64
           " returned_stride=%u returned_offset=%u original_dev=%ju original_ino=%ju returned_dev=%ju returned_ino=%ju same_fd_object=%d\n",
           pixmap, reply->width, reply->height, reply->modifier,
           xcb_dri3_buffers_from_pixmap_strides(reply)[0], xcb_dri3_buffers_from_pixmap_offsets(reply)[0],
           (uintmax_t)original.st_dev, (uintmax_t)original.st_ino, (uintmax_t)returned.st_dev,
           (uintmax_t)returned.st_ino, original.st_dev == returned.st_dev && original.st_ino == returned.st_ino);
    require(original.st_dev == returned.st_dev && original.st_ino == returned.st_ino, "returned_same_buffer");
    close(reply_fds[0]);
    free(reply);
    free(error);
    return pixmap;
}

static void destroy_producer(struct producer *source) {
    require(eglMakeCurrent(source->display, EGL_NO_SURFACE, EGL_NO_SURFACE, source->context), "producer_cleanup_current");
    glDeleteFramebuffers(1, &source->framebuffer);
    glDeleteRenderbuffers(1, &source->renderbuffer);
    PFNEGLDESTROYIMAGEKHRPROC destroy_image = (PFNEGLDESTROYIMAGEKHRPROC)eglGetProcAddress("eglDestroyImageKHR");
    require(destroy_image && destroy_image(source->display, source->image), "producer_image_destroy");
    require(eglMakeCurrent(source->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT), "producer_clear_current");
    require(eglDestroyContext(source->display, source->context), "producer_context_destroy");
    require(eglTerminate(source->display), "producer_terminate");
    gbm_bo_destroy(source->buffer);
    gbm_device_destroy(source->device);
    close(source->device_fd);
}

static GLuint shader(GLenum type, const char *source) {
    GLuint object = glCreateShader(type);
    glShaderSource(object, 1, &source, NULL);
    glCompileShader(object);
    GLint compiled = 0;
    glGetShaderiv(object, GL_COMPILE_STATUS, &compiled);
    require(compiled, "shader_compile");
    return object;
}

static void read_texture(GLuint framebuffer, GLuint program,
                         const unsigned char *expected, const char *stage) {
    const GLfloat vertices[] = {-1, -1, 1, -1, -1, 1, 1, 1};
    glBindFramebuffer(GL_FRAMEBUFFER, framebuffer);
    glViewport(0, 0, 3, 1);
    glDisable(GL_DITHER);
    glUseProgram(program);
    glUniform1i(glGetUniformLocation(program, "image"), 0);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, vertices);
    glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
    unsigned char pixels[12] = {0};
    glReadPixels(0, 0, 3, 1, GL_RGBA, GL_UNSIGNED_BYTE, pixels);
    glFinish();
    if (memcmp(pixels, expected, sizeof pixels) != 0) {
        fprintf(stderr, "egl_pixmap mismatch=%s pixels=", stage);
        for (size_t i = 0; i < sizeof pixels; ++i) fprintf(stderr, "%02x", pixels[i]);
        fputc('\n', stderr);
    }
    require(!x_error && glGetError() == GL_NO_ERROR &&
            memcmp(pixels, expected, sizeof pixels) == 0, stage);
}

int main(int argc, char **argv) {
    const int glx_only = argc == 2 &&
        (strcmp(argv[1], "--glx-imported") == 0 || strcmp(argv[1], "--glx-imported-wire-implicit") == 0);
    const int wire_implicit = argc == 2 &&
        (strcmp(argv[1], "--imported-wire-implicit") == 0 || strcmp(argv[1], "--glx-imported-wire-implicit") == 0);
    const int imported_cross_connection = argc == 2 && strcmp(argv[1], "--imported") == 0;
    const int imported = imported_cross_connection || wire_implicit || glx_only ||
        (argc == 2 && strcmp(argv[1], "--imported-same-connection") == 0);
    const int cross_connection = imported_cross_connection ||
        (argc == 2 && strcmp(argv[1], "--cross-connection") == 0);
    require(argc == 1 || cross_connection || imported, "arguments");
    Display *xdisplay = XOpenDisplay(NULL);
    require(xdisplay != NULL, "connect");
    XSetErrorHandler(record_error);
    Display *owner = cross_connection ? XOpenDisplay(NULL) : xdisplay;
    require(owner != NULL, "owner_connection");
    printf("egl_pixmap cross_connection=%d\n", cross_connection);
    EGLDisplay display = eglGetDisplay((EGLNativeDisplayType)xdisplay);
    require(display != EGL_NO_DISPLAY, "get_display");
    require(eglInitialize(display, NULL, NULL), "initialize");
    printf("egl_pixmap vendor=%s version=%s client_apis=%s\n",
           eglQueryString(display, EGL_VENDOR), eglQueryString(display, EGL_VERSION),
           eglQueryString(display, EGL_CLIENT_APIS));
    /* Pixmap binding requires RGBA8 and a bind-capable pixmap config. */
    const EGLint pixmap_attributes[] = {
        EGL_BUFFER_SIZE, 32, EGL_ALPHA_SIZE, 8, EGL_BLUE_SIZE, 8,
        EGL_GREEN_SIZE, 8, EGL_RED_SIZE, 8, EGL_SURFACE_TYPE, EGL_PIXMAP_BIT,
        EGL_BIND_TO_TEXTURE_RGBA, EGL_TRUE, EGL_NONE
    };
    EGLConfig pixmap_config = NULL;
    EGLint count = 0;
    EGLBoolean selected = eglChooseConfig(display, pixmap_attributes, &pixmap_config, 1, &count);
    printf("egl_pixmap choose_return=%u matching_configs=%d\n", selected, count);
    require(selected == EGL_TRUE && count > 0, "matching_pixmap_config");

    struct producer source = {0};
    Pixmap pixmap;
    GC gc = NULL;
    if (imported) {
        source = make_producer();
        paint_producer(&source, 0);
        pixmap = import_producer(owner, &source, wire_implicit);
    } else {
        pixmap = XCreatePixmap(owner, DefaultRootWindow(owner), 3, 1, 32);
        gc = XCreateGC(owner, pixmap, 0, NULL);
        XSetForeground(owner, gc, 0xff654321);
        XFillRectangle(owner, pixmap, gc, 0, 0, 3, 1);
        XSync(owner, False);
        require(!x_error, "cpu_pixmap");
    }
    if (glx_only) {
        const int glx_config_attributes[] = {
            GLX_RENDER_TYPE, GLX_RGBA_BIT, GLX_DRAWABLE_TYPE, GLX_PIXMAP_BIT,
            GLX_RED_SIZE, 8, GLX_GREEN_SIZE, 8, GLX_BLUE_SIZE, 8,
            GLX_ALPHA_SIZE, 8, GLX_BIND_TO_TEXTURE_RGBA_EXT, True, None
        };
        int glx_count = 0;
        GLXFBConfig *configs = glXChooseFBConfig(xdisplay, DefaultScreen(xdisplay),
                                                glx_config_attributes, &glx_count);
        GLXFBConfig chosen = NULL;
        for (int i = 0; configs && i < glx_count; ++i) {
            XVisualInfo *visual = glXGetVisualFromFBConfig(xdisplay, configs[i]);
            if (visual && visual->depth == 32) chosen = configs[i];
            if (visual) XFree(visual);
            if (chosen) break;
        }
        require(chosen != NULL, "glx_matching_config");
        const int glx_attributes[] = {GLX_TEXTURE_FORMAT_EXT, GLX_TEXTURE_FORMAT_RGBA_EXT,
            GLX_TEXTURE_TARGET_EXT, GLX_TEXTURE_2D_EXT, None};
        GLXPixmap drawable = glXCreatePixmap(xdisplay, chosen, pixmap, glx_attributes);
        printf("egl_pixmap glx_create_pixmap=0x%lx wire_implicit=%d\n", drawable, wire_implicit);
        XSync(xdisplay, False);
        require(drawable != None && !x_error, "glx_create_pixmap");
        glXDestroyPixmap(xdisplay, drawable);
        XFree(configs);
        require(eglTerminate(display), "terminate");
        XFreePixmap(owner, pixmap);
        XSync(owner, False);
        require(!x_error, "cleanup");
        destroy_producer(&source);
        if (cross_connection) XCloseDisplay(owner);
        XCloseDisplay(xdisplay);
        puts("egl_pixmap result=pass glx_initialization=complete");
        return 0;
    }
    const EGLint surface_attributes[] = {
        EGL_TEXTURE_FORMAT, EGL_TEXTURE_RGBA, EGL_TEXTURE_TARGET, EGL_TEXTURE_2D, EGL_NONE
    };
    EGLSurface surface = eglCreatePixmapSurface(display, pixmap_config,
                                                (EGLNativePixmapType)pixmap, surface_attributes);
    require(surface != EGL_NO_SURFACE, "create_pixmap_surface");

    /* ANGLE separates bind-capable pixmap configs from pbuffer configs. */
    const EGLint context_config_attributes[] = {
        EGL_SURFACE_TYPE, EGL_PBUFFER_BIT, EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
        EGL_RED_SIZE, 8, EGL_GREEN_SIZE, 8, EGL_BLUE_SIZE, 8, EGL_ALPHA_SIZE, 8, EGL_NONE
    };
    EGLConfig context_config = NULL;
    require(eglChooseConfig(display, context_config_attributes, &context_config, 1, &count) && count,
            "matching_context_config");
    require(eglBindAPI(EGL_OPENGL_ES_API), "bind_api");
    const EGLint context_attributes[] = {EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE};
    EGLContext context = eglCreateContext(display, context_config, EGL_NO_CONTEXT, context_attributes);
    require(context != EGL_NO_CONTEXT, "create_context");
    const EGLint pbuffer_attributes[] = {EGL_WIDTH, 3, EGL_HEIGHT, 1, EGL_NONE};
    EGLSurface pbuffer = eglCreatePbufferSurface(display, context_config, pbuffer_attributes);
    require(pbuffer != EGL_NO_SURFACE, "create_pbuffer");
    require(eglMakeCurrent(display, pbuffer, pbuffer, context), "make_current");

    GLuint vertex = shader(GL_VERTEX_SHADER,
        "attribute vec2 position; varying vec2 uv; void main(){uv=(position+1.0)*0.5;gl_Position=vec4(position,0,1);}");
    GLuint fragment = shader(GL_FRAGMENT_SHADER,
        "precision mediump float; varying vec2 uv; uniform sampler2D image; void main(){gl_FragColor=texture2D(image,uv);}");
    GLuint program = glCreateProgram();
    glAttachShader(program, vertex);
    glAttachShader(program, fragment);
    glBindAttribLocation(program, 0, "position");
    glLinkProgram(program);
    GLint linked = 0;
    glGetProgramiv(program, GL_LINK_STATUS, &linked);
    require(linked, "program_link");
    GLuint output, framebuffer, texture;
    glGenTextures(1, &output);
    glBindTexture(GL_TEXTURE_2D, output);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, 3, 1, 0, GL_RGBA, GL_UNSIGNED_BYTE, NULL);
    glGenFramebuffers(1, &framebuffer);
    glBindFramebuffer(GL_FRAMEBUFFER, framebuffer);
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, output, 0);
    require(glCheckFramebufferStatus(GL_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE, "readback_framebuffer");
    glGenTextures(1, &texture);
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, texture);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    const unsigned char initial[] = {0x65,0x43,0x21,0xff, 0x65,0x43,0x21,0xff, 0x65,0x43,0x21,0xff};
    const unsigned char dirty[] = {0x65,0x43,0x21,0xff, 0xab,0xcd,0xef,0xff, 0x65,0x43,0x21,0xff};
    require(eglBindTexImage(display, surface, EGL_BACK_BUFFER), "bind_initial");
    read_texture(framebuffer, program, initial, "initial_exact");
    require(eglReleaseTexImage(display, surface, EGL_BACK_BUFFER), "release_initial");
    if (imported) {
        paint_producer(&source, 1);
        require(eglMakeCurrent(display, pbuffer, pbuffer, context), "consumer_current_after_producer");
    } else {
        XSetForeground(owner, gc, 0xffabcdef);
        XFillRectangle(owner, pixmap, gc, 1, 0, 1, 1);
        XSync(owner, False);
    }
    require(eglBindTexImage(display, surface, EGL_BACK_BUFFER), "bind_dirty");
    read_texture(framebuffer, program, dirty, "dirty_exact");
    require(eglReleaseTexImage(display, surface, EGL_BACK_BUFFER), "release_dirty");

    glDeleteTextures(1, &texture);
    glDeleteTextures(1, &output);
    glDeleteFramebuffers(1, &framebuffer);
    glDeleteProgram(program);
    glDeleteShader(vertex);
    glDeleteShader(fragment);
    require(eglDestroySurface(display, surface), "destroy_pixmap_surface");
    require(eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT), "clear_current");
    require(eglDestroySurface(display, pbuffer), "destroy_pbuffer");
    require(eglDestroyContext(display, context), "destroy_context");
    require(eglTerminate(display), "terminate");
    if (gc) XFreeGC(owner, gc);
    XFreePixmap(owner, pixmap);
    XSync(owner, False);
    require(!x_error, "cleanup");
    if (imported) destroy_producer(&source);
    if (cross_connection) XCloseDisplay(owner);
    XCloseDisplay(xdisplay);
    puts("egl_pixmap result=pass initial=exact dirty=exact");
    return 0;
}
