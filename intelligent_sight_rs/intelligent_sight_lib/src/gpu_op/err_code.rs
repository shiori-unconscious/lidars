pub struct ErrCode {
    err_code: &'static [(usize, &'static str)],
}

impl ErrCode {
    pub fn get(&self, code: usize) -> Option<&&'static str> {
        self.err_code
            .iter()
            .find(|(c, _)| *c == code)
            .map(|(_, name)| name)
    }
    pub const fn new(err_code: &'static [(usize, &'static str)]) -> Self {
        ErrCode { err_code }
    }
}

pub const TRT_ERR_NAME: ErrCode = ErrCode::new(&[
    (10000, "TRT_CREATE_ENGINE_FAIL"),
    (10001, "TRT_CREATE_RUNTIME_FAIL"),
    (10002, "TRT_CREATE_CONTEXT_FAIL"),
    (10003, "TRT_READ_ENGINE_FILE_FAIL"),
    (10004, "TRT_INFER_FAIL"),
    (10005, "TRT_DESTROY_ENGINE_FAIL"),
    (10006, "TRT_CREATE_CUDASTREAM_FAIL"),
    (10007, "TRT_ENGINE_NOT_INITIALIZED"),
]);

pub const CUDA_ERR_NAME: ErrCode = ErrCode::new(&[
    (
        0,
        r##"cudaSuccess:
         The API call returned with no errors. In the case of query calls, this also means that the operation being queried is complete (see ::cudaEventQuery"() and ::cudaStreamQuery"()).cudaSuccess"##,
    ),
    (
        1,
        r##"cudaErrorInvalidValue:
         This indicates that one or more of the parameters passed to the API call is not within an acceptable range of values."##,
    ),
    (
        2,
        r##"cudaErrorMemoryAllocation:
 `        The API call failed because it was unable to allocate enough memory or other resources to perform the requested operation."##,
    ),
    (
        3,
        r##"cudaErrorInitializationError:
`         The API call failed because the "CUDA" driver and runtime could not be initialized."##,
    ),
    (
        4,
        r##"cudaErrorCudartUnloading:`
         This indicates that a "CUDA" Runtime API call cannot be executed because it is being called during process shut down, at a point in time after "CUDA" driver has been unloaded."##,
    ),
    (
        5,
        r##"cudaErrorProfilerDisabled:
 `        This indicates profiler is not initialized for this run. This can happen when the application is running with external profiling tools like visual profiler."##,
    ),
    (
        6,
        r##"cudaErrorProfilerNotInitialized:
 `        !!DEPRECATED!! This error return is deprecated as of "CUDA" 5.0. It is no longer an error to attempt to enable/disable the profiling via ::cudaProfilerStart" or ::cudaProfilerStop" without initialization."##,
    ),
    (
        7,
        r##"cudaErrorProfilerAlreadyStarted:
 `        !!DEPRECATED!! This error return is deprecated as of "CUDA" 5.0. It is no longer an error to call cudaProfilerStart"() when profiling is already enabled."##,
    ),
    (
        8,
        r##"cudaErrorProfilerAlreadyStopped:
 `        !!DEPRECATED!! This error return is deprecated as of "CUDA" 5.0. It is no longer an error to call cudaProfilerStop"() when profiling is already disabled."##,
    ),
    (
        9,
        r##"cudaErrorInvalidConfiguration:
 `        This indicates that a kernel launch is requesting resources that can never be satisfied by the current device. Requesting more shared memory per block than the device supports will trigger this error, as will requesting too many threads or blocks. See ::cudaDeviceProp" for more device limitations."##,
    ),
    (
        12,
        r##"cudaErrorInvalidPitchValue:`
         This indicates that one or more of the pitch-related parameters passed to the API call is not within the acceptable range for pitch."##,
    ),
    (
        13,
        r##"cudaErrorInvalidSymbol:`
         This indicates that the symbol name/identifier passed to the API call is not a valid name or identifier."##,
    ),
    (
        16,
        r##"cudaErrorInvalidHostPointer:
 `        This indicates that at least one host pointer passed to the API call is not a valid host pointer. !!DEPRECATED!! This error return is deprecated as of "CUDA" 10.1."##,
    ),
    (
        17,
        r##"cudaErrorInvalidDevicePointer:`
         This indicates that at least one device pointer passed to the API call is not a valid device pointer. !!DEPRECATED!! This error return is deprecated as of "CUDA" 10.1."##,
    ),
    (
        18,
        r##"cudaErrorInvalidTexture:`
         This indicates that the texture passed to the API call is not a valid texture."##,
    ),
    (
        19,
        r##"cudaErrorInvalidTextureBinding:
 `        This indicates that the texture binding is not valid. This occurs if you call ::cudaGetTextureAlignmentOffset"() with an unbound texture."##,
    ),
    (
        20,
        r##"cudaErrorInvalidChannelDescriptor:
 `        This indicates that the channel descriptor passed to the API call is not valid. This occurs if the format is not one of the formats specified by ::cudaChannelFormatKind", or if one of the dimensions is invalid."##,
    ),
    (
        21,
        r##"cudaErrorInvalidMemcpyDirection:
`         This indicates that the direction of the memcpy passed to the API call is not one of the types specified by ::cudaMemcpyKind"."##,
    ),
    (
        22,
        r##"cudaErrorAddressOfConstant:
 `        This indicated that the user has taken the address of a constant variable, which was forbidden up until the "CUDA" 3.1 release. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Variables in constant memory may now have their address taken by the runtime via ::cudaGetSymbolAddress"()."##,
    ),
    (
        23,
        r##"cudaErrorTextureFetchFailed:`
         This indicated that a texture fetch was not able to be performed. This was previously used for device emulation of texture operations. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
    ),
    (
        24,
        r##"cudaErrorTextureNotBound:
 `        This indicated that a texture was not bound for access. This was previously used for device emulation of texture operations. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
    ),
    (
        25,
        r##"cudaErrorSynchronizationError:
 `        This indicated that a synchronization operation had failed. This was previously used for some device emulation functions. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
    ),
    (
        26,
        r##"cudaErrorInvalidFilterSetting:`
         This indicates that a non-float texture was being accessed with linear filtering. This is not supported by "CUDA"."##,
    ),
    (
        27,
        r##"cudaErrorInvalidNormSetting:
 `        This indicates that an attempt was made to read a non-float texture as a normalized float. This is not supported by "CUDA"."##,
    ),
    (
        28,
        r##"cudaErrorMixedDeviceExecution:`
         Mixing of device and device emulation code was not allowed. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
    ),
    (
        31,
        r##"cudaErrorNotYetImplemented:
 `        This indicates that the API call is not yet implemented. Production releases of "CUDA" will never return this error. !!DEPRECATED!! This error return is deprecated as of "CUDA" 4.1."##,
    ),
    (
        32,
        r##"cudaErrorMemoryValueTooLarge:`
         This indicated that an emulated device pointer exceeded the 32-bit address range. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
    ),
    (
        34,
        r##"cudaErrorStubLibrary:`
         This indicates that the "CUDA" driver that the application has loaded is a stub library. Applications that run with the stub rather than a real driver loaded will result in "CUDA" API returning this error."##,
    ),
    (
        35,
        r##"cudaErrorInsufficientDriver:
 `        This indicates that the installed NVIDIA "CUDA" driver is older than the "CUDA" runtime library. This is not a supported configuration. Users should install an updated NVIDIA display driver to allow the application to run."##,
    ),
    (
        36,
        r##"cudaErrorCallRequiresNewerDriver:`
         This indicates that the API call requires a newer "CUDA" driver than the one currently installed. Users should install an updated NVIDIA "CUDA" driver to allow the API call to succeed."##,
    ),
    (
        37,
        r##"cudaErrorInvalidSurface:
 `        This indicates that the surface passed to the API call is not a valid surface."##,
    ),
    (
        43,
        r##"cudaErrorDuplicateVariableName:
 `        This indicates that multiple global or constant variables (across separate "CUDA" source files in the application) share the same string name."##,
    ),
    (
        44,
        r##"cudaErrorDuplicateTextureName:
 `        This indicates that multiple textures (across separate "CUDA" source files in the application) share the same string name."##,
    ),
    (
        45,
        r##"cudaErrorDuplicateSurfaceName:`
         This indicates that multiple surfaces (across separate "CUDA" source files in the application) share the same string name."##,
    ),
    (
        46,
        r##"cudaErrorDevicesUnavailable:
 `        This indicates that all "CUDA" devices are busy or unavailable at the current time. Devices are often busy/unavailable due to use of ::cudaComputeModeProhibited", ::cudaComputeModeExclusiveProcess", or when long running "CUDA" kernels have filled up the GPU and are blocking new work from starting. They can also be unavailable due to memory constraints on a device that already has active "CUDA" work being performed."##,
    ),
    (
        49,
        r##"cudaErrorIncompatibleDriverContext:
 `        This indicates that the current context is not compatible with this the "CUDA" Runtime. This can only occur if you are using "CUDA" Runtime/Driver interoperability and have created an existing Driver context using the driver API. The Driver context may be incompatible either because the Driver context was created using an older version  of the API, because the Runtime API call expects a primary driver  context and the Driver context is not primary, or because the Driver  context has been destroyed. Please see \ref "CUDART"_DRIVER "Interactions  with the "CUDA" Driver API" for more information."##,
    ),
    (
        52,
        r##"cudaErrorMissingConfiguration:`
         The device function being invoked (usually via ::cudaLaunchKernel"()) was not previously configured via the ::cudaConfigureCall"() function."##,
    ),
    (
        53,
        r##"cudaErrorPriorLaunchFailure:
 `        This indicated that a previous kernel launch failed. This was previously used for device emulation of kernel launches. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
    ),
    (
        65,
        r##"cudaErrorLaunchMaxDepthExceeded:
`         This error indicates that a device runtime grid launch did not occur  because the depth of the child grid would exceed the maximum supported number of nested grid launches. "##,
    ),
    (
        66,
        r##"cudaErrorLaunchFileScopedTex:
 `        This error indicates that a grid launch did not occur because the kernel  uses file-scoped textures which are unsupported by the device runtime.  Kernels launched via the device runtime only support textures created with  the Texture Object API's."##,
    ),
    (
        67,
        r##"cudaErrorLaunchFileScopedSurf:`
         This error indicates that a grid launch did not occur because the kernel  uses file-scoped surfaces which are unsupported by the device runtime. Kernels launched via the device runtime only support surfaces created with the Surface Object API's."##,
    ),
    (
        68,
        r##"cudaErrorSyncDepthExceeded:
 `        This error indicates that a call to ::cudaDeviceSynchronize" made from the device runtime failed because the call was made at grid depth greater than than either the default (2 levels of grids) or user specified device limit ::cudaLimitDevRuntimeSyncDepth". To be able to synchronize on launched grids at a greater depth successfully, the maximum nested depth at which ::cudaDeviceSynchronize" will be called must be specified with the ::cudaLimitDevRuntimeSyncDepth" limit to the ::cudaDeviceSetLimit" api before the host-side launch of a kernel using the device runtime. Keep in mind that additional levels of sync depth require the runtime to reserve large amounts of device memory that cannot be used for user allocations. Note that ::cudaDeviceSynchronize" made from device runtime is only supported on devices of compute capability < 9.0."##,
    ),
    (
        69,
        r##"cudaErrorLaunchPendingCountExceeded:
 `        This error indicates that a device runtime grid launch failed because the launch would exceed the limit ::cudaLimitDevRuntimePendingLaunchCount". For this launch to proceed successfully, ::cudaDeviceSetLimit" must be called to set the ::cudaLimitDevRuntimePendingLaunchCount" to be higher  than the upper bound of outstanding launches that can be issued to the device runtime. Keep in mind that raising the limit of pending device runtime launches will require the runtime to reserve device memory that cannot be used for user allocations."##,
    ),
    (
        98,
        r##"cudaErrorInvalidDeviceFunction:`
         The requested device function does not exist or is not compiled for the proper device architecture."##,
    ),
    (
        100,
        r##"cudaErrorNoDevice:`
         This indicates that no "CUDA"-capable devices were detected by the installed "CUDA" driver."##,
    ),
    (
        101,
        r##"cudaErrorInvalidDevice:`
         This indicates that the device ordinal supplied by the user does not correspond to a valid "CUDA" device or that the action requested is invalid for the specified device."##,
    ),
    (
        102,
        r##"cudaErrorDeviceNotLicensed:
 `        This indicates that the device doesn't have a valid Grid License."##,
    ),
    (
        103,
        r##"cudaErrorSoftwareValidityNotEstablished:`
         By default, the "CUDA" runtime may perform a minimal set of self-tests, as well as "CUDA" driver tests, to establish the validity of both. Introduced in "CUDA" 11.2, this error return indicates that at least one of these tests has failed and the validity of either the runtime or the driver could not be established."##,
    ),
    (
        127,
        r##"cudaErrorStartupFailure:
`         This indicates an internal startup failure in the "CUDA" runtime."##,
    ),
    (
        200,
        r##"cudaErrorInvalidKernelImage:`
         This indicates that the device kernel image is invalid."##,
    ),
    (
        201,
        r##"cudaErrorDeviceUninitialized:`
         This most frequently indicates that there is no context bound to the current thread. This can also be returned if the context passed to an API call is not a valid handle (such as a context that has had ::cuCtxDestroy() invoked on it). This can also be returned if a user mixes different API versions (i.e. 3010 context with 3020 API calls). See ::cuCtxGetApiVersion() for more details."##,
    ),
    (
        205,
        r##"cudaErrorMapBufferObjectFailed:
 `        This indicates that the buffer object could not be mapped."##,
    ),
    (
        206,
        r##"cudaErrorUnmapBufferObjectFailed:
 `        This indicates that the buffer object could not be unmapped."##,
    ),
    (
        207,
        r##"cudaErrorArrayIsMapped:`
         This indicates that the specified array is currently mapped and thus cannot be destroyed."##,
    ),
    (
        208,
        r##"cudaErrorAlreadyMapped:
 `        This indicates that the resource is already mapped."##,
    ),
    (
        209,
        r##"cudaErrorNoKernelImageForDevice:
 `        This indicates that there is no kernel image available that is suitable for the device. This can occur when a user specifies code generation options for a particular "CUDA" source file that do not include the corresponding device configuration."##,
    ),
    (
        210,
        r##"cudaErrorAlreadyAcquired:`
         This indicates that a resource has already been acquired."##,
    ),
    (
        211,
        r##"cudaErrorNotMapped:`
         This indicates that a resource is not mapped."##,
    ),
    (
        212,
        r##"cudaErrorNotMappedAsArray:
 `        This indicates that a mapped resource is not available for access as an array."##,
    ),
    (
        213,
        r##"cudaErrorNotMappedAsPointer:
 `        This indicates that a mapped resource is not available for access as a pointer."##,
    ),
    (
        214,
        r##"cudaErrorECCUncorrectable:`
         This indicates that an uncorrectable ECC error was detected during execution."##,
    ),
    (
        215,
        r##"cudaErrorUnsupportedLimit:`
         This indicates that the ::cudaLimit" passed to the API call is not supported by the active device."##,
    ),
    (
        216,
        r##"cudaErrorDeviceAlreadyInUse:
 `        This indicates that a call tried to access an exclusive-thread device that  is already in use by a different thread."##,
    ),
    (
        217,
        r##"cudaErrorPeerAccessUnsupported:`
         This error indicates that P2P access is not supported across the given devices."##,
    ),
    (
        218,
        r##"cudaErrorInvalidPtx:
 `        A PTX compilation failed. The runtime may fall back to compiling PTX if an application does not contain a suitable binary for the current device."##,
    ),
    (
        219,
        r##"cudaErrorInvalidGraphicsContext:
 `        This indicates an error with the OpenGL or DirectX context."##,
    ),
    (
        220,
        r##"cudaErrorNvlinkUncorrectable:`
         This indicates that an uncorrectable NVLink error was detected during the execution."##,
    ),
    (
        221,
        r##"cudaErrorJitCompilerNotFound:
`         This indicates that the PTX JIT compiler library was not found. The JIT Compiler library is used for PTX compilation. The runtime may fall back to compiling PTX if an application does not contain a suitable binary for the current device."##,
    ),
    (
        222,
        r##"cudaErrorUnsupportedPtxVersion:
`         This indicates that the provided PTX was compiled with an unsupported toolchain. The most common reason for this, is the PTX was generated by a compiler newer than what is supported by the "CUDA" driver and PTX JIT compiler."##,
    ),
    (
        223,
        r##"cudaErrorJitCompilationDisabled:
 `        This indicates that the JIT compilation was disabled. The JIT compilation compiles PTX. The runtime may fall back to compiling PTX if an application does not contain a suitable binary for the current device."##,
    ),
    (
        224,
        r##"cudaErrorUnsupportedExecAffinity:
 `        This indicates that the provided execution affinity is not supported by the device."##,
    ),
    (
        225,
        r##"cudaErrorUnsupportedDevSideSync:
 `        This indicates that the code to be compiled by the PTX JIT contains unsupported call to cudaDeviceSynchronize"."##,
    ),
    (
        300,
        r##"cudaErrorInvalidSource:`
         This indicates that the device kernel source is invalid."##,
    ),
    (
        301,
        r##"cudaErrorFileNotFound:
 `        This indicates that the file specified was not found."##,
    ),
    (
        302,
        r##"cudaErrorSharedObjectSymbolNotFound:
 `        This indicates that a link to a shared object failed to resolve."##,
    ),
    (
        303,
        r##"cudaErrorSharedObjectInitFailed:`
         This indicates that initialization of a shared object failed."##,
    ),
    (
        304,
        r##"cudaErrorOperatingSystem:
 `        This error indicates that an OS call failed."##,
    ),
    (
        400,
        r##"cudaErrorInvalidResourceHandle:
 `        This indicates that a resource handle passed to the API call was not valid. Resource handles are opaque types like ::cudaStream"_t and ::cudaEvent"_t."##,
    ),
    (
        401,
        r##"cudaErrorIllegalState:`
         This indicates that a resource required by the API call is not in a valid state to perform the requested operation."##,
    ),
    (
        402,
        r##"cudaErrorLossyQuery:
 `        This indicates an attempt was made to introspect an object in a way that would discard semantically important information. This is either due to the object using funtionality newer than the API version used to introspect it or omission of optional return arguments."##,
    ),
    (
        500,
        r##"cudaErrorSymbolNotFound:`
         This indicates that a named symbol was not found. Examples of symbols are global/constant variable names, driver function names, texture names, and surface names."##,
    ),
    (
        600,
        r##"cudaErrorNotReady:
         This indicates that asynchronous operations issued previously have not completed yet. This result is not actually an error, but must be indicated differently than ::cudaSuccess" (which indicates completion). Calls that may return this value include ::cudaEventQuery"() and ::cudaStreamQuery"()."##,
    ),
    (
        700,
        r##"cudaErrorIllegalAddress:
         The device encountered a load or store instruction on an invalid memory address. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        701,
        r##"cudaErrorLaunchOutOfResources:
         This indicates that a launch did not occur because it did not have appropriate resources. Although this error is similar to ::cudaErrorInvalidConfiguration, this error usually indicates that the user has attempted to pass too many arguments to the device kernel, or the kernel launch specifies too many threads for the kernel's register count."##,
    ),
    (
        702,
        r##"cudaErrorLaunchTimeout:
         This indicates that the device kernel took too long to execute. This can only occur if timeouts are enabled - see the device property \ref ::cudaDeviceProp"::kernelExecTimeoutEnabled "kernelExecTimeoutEnabled" for more information. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        703,
        r##"cudaErrorLaunchIncompatibleTexturing:
         This error indicates a kernel launch that uses an incompatible texturing mode."##,
    ),
    (
        704,
        r##"cudaErrorPeerAccessAlreadyEnabled:
         This error indicates that a call to ::cudaDeviceEnablePeerAccess"() is trying to re-enable peer addressing on from a context which has already had peer addressing enabled."##,
    ),
    (
        705,
        r##"cudaErrorPeerAccessNotEnabled:
         This error indicates that ::cudaDeviceDisablePeerAccess"() is trying to  disable peer addressing which has not been enabled yet via  ::cudaDeviceEnablePeerAccess"()."##,
    ),
    (
        708,
        r##"cudaErrorSetOnActiveProcess:
         This indicates that the user has called ::cudaSetValidDevices"(), ::cudaSetDeviceFlags"(), ::cudaD"3D9SetDirect3DDevice(), ::cudaD"3D10SetDirect3DDevice, ::cudaD"3D11SetDirect3DDevice(), or ::cudaVDPAUSetVDPAUDevice"() after initializing the "CUDA" runtime by calling non-device management operations (allocating memory and launching kernels are examples of non-device management operations). This error can also be returned if using runtime/driver interoperability and there is an existing ::CUcontext active on the host thread."##,
    ),
    (
        709,
        r##"cudaErrorContextIsDestroyed:
         This error indicates that the context current to the calling thread has been destroyed using ::cuCtxDestroy, or is a primary context which has not yet been initialized."##,
    ),
    (
        710,
        r##"cudaErrorAssert:
         An assert triggered in device code during kernel execution. The device cannot be used again. All existing allocations are invalid. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        711,
        r##"cudaErrorTooManyPeers:
         This error indicates that the hardware resources required to enable peer access have been exhausted for one or more of the devices  passed to ::cudaEnablePeerAccess"()."##,
    ),
    (
        712,
        r##"cudaErrorHostMemoryAlreadyRegistered:
         This error indicates that the memory range passed to ::cudaHostRegister"() has already been registered."##,
    ),
    (
        713,
        r##"cudaErrorHostMemoryNotRegistered:
         This error indicates that the pointer passed to ::cudaHostUnregister"() does not correspond to any currently registered memory region."##,
    ),
    (
        714,
        r##"cudaErrorHardwareStackError:
         Device encountered an error in the call stack during kernel execution, possibly due to stack corruption or exceeding the stack size limit. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        715,
        r##"cudaErrorIllegalInstruction:
         The device encountered an illegal instruction during kernel execution This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        716,
        r##"cudaErrorMisalignedAddress:
         The device encountered a load or store instruction on a memory address which is not aligned. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        717,
        r##"cudaErrorInvalidAddressSpace:
         While executing a kernel, the device encountered an instruction which can only operate on memory locations in certain address spaces (global, shared, or local), but was supplied a memory address not belonging to an allowed address space. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        718,
        r##"cudaErrorInvalidPc:
         The device encountered an invalid program counter. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        719,
        r##"cudaErrorLaunchFailure:
         An exception occurred on the device while executing a kernel. Common causes include dereferencing an invalid device pointer and accessing out of bounds shared memory. Less common cases can be system specific - more information about these cases can be found in the system specific user guide. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        720,
        r##"cudaErrorCooperativeLaunchTooLarge:
         This error indicates that the number of blocks launched per grid for a kernel that was launched via either ::cudaLaunchCooperativeKernel" or ::cudaLaunchCooperativeKernelMultiDevice" exceeds the maximum number of blocks as allowed by ::cudaOccupancyMaxActiveBlocksPerMultiprocessor" or ::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags" times the number of multiprocessors as specified by the device attribute ::cudaDevAttrMultiProcessorCount"."##,
    ),
    (
        800,
        r##"cudaErrorNotPermitted:
         This error indicates the attempted operation is not permitted."##,
    ),
    (
        801,
        r##"cudaErrorNotSupported:
         This error indicates the attempted operation is not supported on the current system or device."##,
    ),
    (
        802,
        r##"cudaErrorSystemNotReady:
         This error indicates that the system is not yet ready to start any "CUDA" work.  To continue using "CUDA", verify the system configuration is in a valid state and all required driver daemons are actively running. More information about this error can be found in the system specific user guide."##,
    ),
    (
        803,
        r##"cudaErrorSystemDriverMismatch:
         This error indicates that there is a mismatch between the versions of the display driver and the "CUDA" driver. Refer to the compatibility documentation for supported versions."##,
    ),
    (
        804,
        r##"cudaErrorCompatNotSupportedOnDevice:
         This error indicates that the system was upgraded to run with forward compatibility but the visible hardware detected by "CUDA" does not support this configuration. Refer to the compatibility documentation for the supported hardware matrix or ensure that only supported hardware is visible during initialization via the "CUDA"_VISIBLE_DEVICES environment variable."##,
    ),
    (
        805,
        r##"cudaErrorMpsConnectionFailed:
         This error indicates that the MPS client failed to connect to the MPS control daemon or the MPS server."##,
    ),
    (
        806,
        r##"cudaErrorMpsRpcFailure:
         This error indicates that the remote procedural call between the MPS server and the MPS client failed."##,
    ),
    (
        807,
        r##"cudaErrorMpsServerNotReady:
         This error indicates that the MPS server is not ready to accept new MPS client requests. This error can be returned when the MPS server is in the process of recovering from a fatal failure."##,
    ),
    (
        808,
        r##"cudaErrorMpsMaxClientsReached:
         This error indicates that the hardware resources required to create MPS client have been exhausted."##,
    ),
    (
        809,
        r##"cudaErrorMpsMaxConnectionsReached:
         This error indicates the the hardware resources required to device connections have been exhausted."##,
    ),
    (
        810,
        r##"cudaErrorMpsClientTerminated:
         This error indicates that the MPS client has been terminated by the server. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        811,
        r##"cudaErrorCdpNotSupported:
         This error indicates, that the program is using "CUDA" Dynamic Parallelism, but the current configuration, like MPS, does not support it."##,
    ),
    (
        812,
        r##"cudaErrorCdpVersionMismatch:
         This error indicates, that the program contains an unsupported interaction between different versions of "CUDA" Dynamic Parallelism."##,
    ),
    (
        900,
        r##"cudaErrorStreamCaptureUnsupported:
         The operation is not permitted when the stream is capturing."##,
    ),
    (
        901,
        r##"cudaErrorStreamCaptureInvalidated:
         The current capture sequence on the stream has been invalidated due to a previous error."##,
    ),
    (
        902,
        r##"cudaErrorStreamCaptureMerge:
         The operation would have resulted in a merge of two independent capture sequences."##,
    ),
    (
        903,
        r##"cudaErrorStreamCaptureUnmatched:
         The capture was not initiated in this stream."##,
    ),
    (
        904,
        r##"cudaErrorStreamCaptureUnjoined:
         The capture sequence contains a fork that was not joined to the primary stream."##,
    ),
    (
        905,
        r##"cudaErrorStreamCaptureIsolation:
         A dependency would have been created which crosses the capture sequence boundary. Only implicit in-stream ordering dependencies are allowed to cross the boundary."##,
    ),
    (
        906,
        r##"cudaErrorStreamCaptureImplicit:
         The operation would have resulted in a disallowed implicit dependency on a current capture sequence from cudaStreamLegacy"."##,
    ),
    (
        907,
        r##"cudaErrorCapturedEvent:
         The operation is not permitted on an event which was last recorded in a capturing stream."##,
    ),
    (
        908,
        r##"cudaErrorStreamCaptureWrongThread:
         A stream capture sequence not initiated with the ::cudaStreamCaptureModeRelaxed" argument to ::cudaStreamBeginCapture" was passed to ::cudaStreamEndCapture" in a different thread."##,
    ),
    (
        909,
        r##"cudaErrorTimeout:
         This indicates that the wait operation has timed out."##,
    ),
    (
        910,
        r##"cudaErrorGraphExecUpdateFailure:
         This error indicates that the graph update was not performed because it included  changes which violated constraints specific to instantiated graph update."##,
    ),
    (
        911,
        r##"cudaErrorExternalDevice:
         This indicates that an async error has occurred in a device outside of "CUDA". If "CUDA" was waiting for an external device's signal before consuming shared data, the external device signaled an error indicating that the data is not valid for consumption. This leaves the process in an inconsistent state and any further "CUDA" work will return the same error. To continue using "CUDA", the process must be terminated and relaunched."##,
    ),
    (
        912,
        r##"cudaErrorInvalidClusterSize:
         This indicates that a kernel launch error has occurred due to cluster misconfiguration."##,
    ),
    (
        999,
        r##"cudaErrorUnknown:
         This indicates that an unknown internal error has occurred."##,
    ),
    (
        10000,
        r##"cudaErrorApiFailureBase:
        Any unhandled "CUDA" driver error is added to this value and returned via the runtime. Production releases of "CUDA" should not return such errors. !!DEPRECATED!! This error return is deprecated as of "CUDA" 4.1."##,
    ),
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errcode() {
        assert_eq!(
            &r##"cudaErrorTextureNotBound:
 `        This indicated that a texture was not bound for access. This was previously used for device emulation of texture operations. !!DEPRECATED!! This error return is deprecated as of "CUDA" 3.1. Device emulation mode was removed with the "CUDA" 3.1 release."##,
            CUDA_ERR_NAME.get(24).unwrap()
        );
        assert_eq!(None, CUDA_ERR_NAME.get(114514));
    }
}
//1280 1024
