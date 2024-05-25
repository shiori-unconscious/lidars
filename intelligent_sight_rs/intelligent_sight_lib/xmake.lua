add_rules("mode.release", "mode.debug")

if is_mode("debug") then
    set_optimize("none")
else 
    if is_mode("release") then 
        set_optimize("fastest")
    end
end

target("camera_wrapper")
    set_kind("shared")
    set_languages("c17")
    set_targetdir("$(projectdir)/clibs")
    add_files("src/cam_op/c_src/*.c")
    if is_os("windows") then
        add_includedirs("$(projectdir)\\linuxSDK_V2.1.0.37\\include")
        add_linkdirs("$(projectdir)\\linuxSDK_V2.1.0.37\\lib")
        add_links("MVCAMSDK_X64")
        add_rules("utils.symbols.export_all")
    end
    if is_os("linux") then
        add_links("MVSDK")
    end

target("cuda_wrapper")
    set_kind("shared")
    set_languages("c++17")
    set_targetdir("$(projectdir)/clibs")
    add_files("src/gpu_op/cu_src/*.cu")
    add_files("src/gpu_op/cxx_src/*.cpp")
    add_cuflags("--extended-lambda")
    add_cugencodes("native")
    if is_os("windows") then
        add_includedirs('C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.4\\include', 'D:\\Program Files (x86)\\TensorRT-10.0.0.6\\include')
        add_linkdirs("C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.4\\lib\\x64", "D:\\Program Files (x86)\\TensorRT-10.0.0.6\\lib")
        add_rules("utils.symbols.export_all")
    end
    if is_os("linux") then
        add_includedirs('/usr/local/cuda/include')
        if os.isdir('/usr/local/cuda/lib') then
            add_linkdirs('/usr/local/cuda/lib')
        end
        if os.isdir('/usr/local/cuda/lib64') then
            add_linkdirs('/usr/local/cuda/lib64')
        end
    end
    add_links("cudart", "cuda", "nvinfer")

-- target("camera_test")
--     set_kind("binary")
--     set_targetdir("$(projectdir)/bin")
--     add_linkdirs("$(projectdir)/linuxSDK_V2.1.0.37/lib")
--     add_links("MVCAMSDK_X64")
--     add_files("src/cam_op/c_src/*.c")
    
--
-- If you want to known more usage about xmake, please see https://xmake.io
--
-- ## FAQ
--
-- You can enter the project directory firstly before building project.
--
--   $ cd projectdir
--
-- 1. How to build project?
--
--   $ xmake
--
-- 2. How to configure project?
--
--   $ xmake f -p [macosx|linux|iphoneos ..] -a [x86_64|i386|arm64 ..] -m [debug|release]
--
-- 3. Where is the build output directory?
--
--   The default output directory is `./build` and you can configure the output directory.
--
--   $ xmake f -o outputdir
--   $ xmake
--
-- 4. How to run and debug target after building project?
--
--   $ xmake run [targetname]
--   $ xmake run -d [targetname]
--
-- 5. How to install target to the system directory or other output directory?
--
--   $ xmake install
--   $ xmake install -o installdir
--
-- 6. Add some frequently-used compilation flags in xmake.lua
--
-- @code
--    -- add debug and release modes
--    add_rules("mode.debug", "mode.release")
--
--    -- add macro definition
--    add_defines("NDEBUG", "_GNU_SOURCE=1")
--
--    -- set warning all as error
--    set_warnings("all", "error")
--
--    -- set language: c99, c++11
--    set_languages("c99", "c++11")
--
--    -- set optimization: none, faster, fastest, smallest
--    set_optimize("fastest")
--
--    -- add include search directories
--    add_includedirs("/usr/include", "/usr/local/include")
--
--    -- add link libraries and search directories
--    add_links("tbox")
--    add_linkdirs("/usr/local/lib", "/usr/lib")
--
--    -- add system link libraries
--    add_syslinks("z", "pthread")
--
--    -- add compilation and link flags
--    add_cxflags("-stdnolib", "-fno-strict-aliasing")
--    add_ldflags("-L/usr/local/lib", "-lpthread", {force = true})
--
-- @endcode
--
