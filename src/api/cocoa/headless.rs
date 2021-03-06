use ContextError;
use CreationError;
use CreationError::OsError;
use GlAttributes;
use GlContext;
use PixelFormatRequirements;
use std::os::raw::c_void;
use std::ptr;

use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use cocoa::base::{id, nil};
use cocoa::appkit::*;
use PixelFormat;

pub struct HeadlessContext {
    width: u32,
    height: u32,
    context: id,
}

impl HeadlessContext {
    pub fn new((width, height): (u32, u32), _pf_reqs: &PixelFormatRequirements,
               _opengl: &GlAttributes<&HeadlessContext>) -> Result<HeadlessContext, CreationError>
    {
        let context = unsafe {
            let attributes = [
                NSOpenGLPFADoubleBuffer as u32,
                NSOpenGLPFAClosestPolicy as u32,
                NSOpenGLPFAColorSize as u32, 24,
                NSOpenGLPFAAlphaSize as u32, 8,
                NSOpenGLPFADepthSize as u32, 24,
                NSOpenGLPFAStencilSize as u32, 8,

                NSOpenGLPFAOpenGLProfile as u32, NSOpenGLProfileVersion3_2Core as u32,
                0
            ];

            let pixelformat = NSOpenGLPixelFormat::alloc(nil).initWithAttributes_(&attributes);
            if pixelformat == nil {
                return Err(OsError(format!("Could not create the pixel format")));
            }
            let context = NSOpenGLContext::alloc(nil).initWithFormat_shareContext_(pixelformat, nil);
            if context == nil {
                return Err(OsError(format!("Could not create the rendering context")));
            }
            context
        };

        let headless = HeadlessContext {
            width: width,
            height: height,
            context: context,
        };

        Ok(headless)
    }
}

impl GlContext for HeadlessContext {
    unsafe fn make_current(&self) -> Result<(), ContextError> {
        self.context.makeCurrentContext();
        Ok(())
    }

    #[inline]
    fn is_current(&self) -> bool {
        true
    }

    #[inline]
    fn get_proc_address(&self, _addr: &str) -> *const () {
        let symbol_name: CFString = _addr.parse().unwrap();
        let framework_name: CFString = "com.apple.opengl".parse().unwrap();
        let framework = unsafe {
            CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef())
        };
        let symbol = unsafe {
            CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef())
        };
        symbol as *const ()
    }

    #[inline]
    fn swap_buffers(&self) -> Result<(), ContextError> {
        unsafe { self.context.flushBuffer(); }
        Ok(())
    }

    #[inline]
    fn get_api(&self) -> ::Api {
        ::Api::OpenGl
    }

    #[inline]
    fn get_pixel_format(&self) -> PixelFormat {
        unimplemented!();
    }
}

unsafe impl Send for HeadlessContext {}
unsafe impl Sync for HeadlessContext {}
