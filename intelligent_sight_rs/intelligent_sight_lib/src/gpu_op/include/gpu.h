#ifndef GPU_WRAPPER_H
#define GPU_WRAPPER_H

#include <NvInfer.h>
#include <cstdint>
#include <cuda_runtime_api.h>
#include <thrust/device_ptr.h>
#include <string>

#define check_status(fun)                \
    do                                   \
    {                                    \
        int ret_status = (fun);          \
        if (ret_status != cudaSuccess)   \
        {                                \
            return (uint16_t)ret_status; \
        }                                \
    } while (0)

extern "C"
{
    uint16_t cuda_malloc(uint32_t size, uint8_t **buffer);
    uint16_t cuda_malloc_managed(uint32_t size, uint8_t **buffer);
    uint16_t cuda_malloc_host(uint32_t size, uint8_t **buffer);
    uint16_t cuda_free(uint8_t *buffer);
    uint16_t cuda_free_host(uint8_t *buffer);
    uint16_t convert_rgb888_3dtensor(uint8_t *input_buffer, float *output_buffer, uint32_t width, uint32_t height);
    uint16_t init_cuda();
    uint16_t destroy_cuda();
    uint16_t transfer_host_to_device(uint8_t *host_mem, uint8_t *device_mem, uint32_t size);
    uint16_t transfer_device_to_host(uint8_t *host_mem, uint8_t *device_mem, uint32_t size);
    uint16_t create_engine(const char *engine_filename, const char *input_name, const char *output_name, uint32_t width, uint32_t height);
    uint16_t create_context();
    uint16_t infer();
    uint16_t release_resources();
    uint16_t set_input(float *input_buffer);
    uint16_t set_output(float *output_buffer);
    uint16_t postprocess_init(uint16_t max_detect, float conf_threshold, float iou_threshold, uint16_t feature_map_size);
    uint16_t postprocess_init_default();
    uint16_t postprocess(float *input_buffer, float *output_buffer, uint16_t *num_detections);
    uint16_t postprocess_destroy();
}

struct PostProcess
{
private:
    uint16_t MAX_DETECT = 25;
    float CONF_THRESHOLD = 0.5;
    float IOU_THRESHOLD = 0.5;
    uint16_t FEATURE_MAP_SIZE = 6300;

    float *transformed, *host_transformed;
    int *indices, *host_indices;
    thrust::device_ptr<int> d_indices;
    thrust::device_ptr<float> d_transformed;
    bool check_iou(float *box1, float *box2);

public:
    PostProcess();
    PostProcess(uint16_t max_detect, float conf_threshold, float iou_threshold, uint16_t feature_map_size);
    uint16_t init();
    uint16_t post_process(float *input_buffer, float *output_buffer, uint16_t *num_detections);
    uint16_t uninit();
};

enum TrtErrCode
{
    TRT_OK = 0,
    TRT_CREATE_ENGINE_FAIL = 10000,
    TRT_CREATE_RUNTIME_FAIL,
    TRT_CREATE_CONTEXT_FAIL,
    TRT_READ_ENGINE_FILE_FAIL,
    TRT_INFER_FAIL,
    TRT_DESTROY_ENGINE_FAIL,
    TRT_CREATE_CUDASTREAM_FAIL,
    TRT_ENGINE_NOT_INITIALIZED,
    TRT_ENGINE_ALREADY_CREATED,
};

class Logger : public nvinfer1::ILogger
{
private:
    const char *log_level[10] = {"[INTERNAL_ERROR]", "[ERROR]", "[WARNING]", "[INFO]", "[VERBOSE]"};

public:
    void log(Severity severity, const char *msg) noexcept override;
};

struct TensorrtInfer
{
private:
    Logger G_LOGGER;
    cudaStream_t CUDA_STREAM;
    nvinfer1::ICudaEngine *M_ENGINE = nullptr;
    nvinfer1::IExecutionContext *CONTEXT = nullptr;
    nvinfer1::IRuntime *RUNTIME = nullptr;
    uint32_t WIDTH, HEIGHT;
    std::string ENGINE_NAME, INPUT_NAME, OUTPUT_NAME;
    // float *INPUT, *OUTPUT;

public:
    TensorrtInfer(const char *engine_filename, const char *input_name, const char *output_name, uint32_t width, uint32_t height);
    ~TensorrtInfer();
    uint16_t release_resources();
    uint16_t create_engine();
    uint16_t infer();
    uint16_t create_context();
    uint16_t set_input(float *input_buffer);
    uint16_t set_output(float *output_buffer);
};

#endif