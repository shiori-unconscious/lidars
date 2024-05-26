#include <cuda_runtime_api.h>
#include <thrust/device_vector.h>
#include <thrust/sort.h>

#include "../include/gpu.h"

// input tensor shape (1, 5, FEATURE_MAP_SIZE)
// 5: 4(xywh) + 1(class)
// output shape (1, FEATURE_MAP_SIZE, 5)
__global__ void transform_results(float *input_buffer, float *output_buffer, uint16_t FEATURE_MAP_SIZE)
{
    int x = blockIdx.x * blockDim.x + threadIdx.x;

    if (x < FEATURE_MAP_SIZE)
    {
        for (int i = 0; i < 5; i++)
        {
            output_buffer[x * 5 + i] = input_buffer[i * FEATURE_MAP_SIZE + x];
        }
    }
}

PostProcess::PostProcess() {}

PostProcess::PostProcess(uint16_t max_detect, float conf_threshold, float iou_threshold, uint16_t feature_map_size) : MAX_DETECT(max_detect), CONF_THRESHOLD(conf_threshold), IOU_THRESHOLD(iou_threshold), FEATURE_MAP_SIZE(feature_map_size) {}

uint16_t PostProcess::init()
{
    check_status(cudaMalloc(&this->transformed, FEATURE_MAP_SIZE * 5 * sizeof(float)));
    check_status(cudaMalloc(&this->indices, FEATURE_MAP_SIZE * sizeof(int)));

    this->d_transformed = thrust::device_ptr<float>(this->transformed);
    this->d_indices = thrust::device_ptr<int>(this->indices);

    check_status(cudaMallocHost(&this->host_transformed, FEATURE_MAP_SIZE * 5 * sizeof(float)));
    check_status(cudaMallocHost(&this->host_indices, FEATURE_MAP_SIZE * sizeof(int)));

    return (uint16_t)cudaSuccess;
}

uint16_t PostProcess::uninit()
{
    check_status(cudaFree(this->transformed));
    check_status(cudaFree(this->indices));

    check_status(cudaFreeHost(this->host_transformed));
    check_status(cudaFreeHost(this->host_indices));

    return (uint16_t)cudaSuccess;
}

bool PostProcess::check_iou(float *box1, float *box2)
{
    float x1 = box1[0];
    float y1 = box1[1];
    float w1 = box1[2];
    float h1 = box1[3];
    float x2 = box2[0];
    float y2 = box2[1];
    float w2 = box2[2];
    float h2 = box2[3];
    float area_inter = fmax(fmin(x1 + w1 / 2, x2 + w2 / 2) - fmax(x1 - w1 / 2, x2 - w2 / 2), 0.0f) * fmax(fmin(y1 + h1 / 2, y2 + h2 / 2) - fmax(y1 - h1 / 2, y2 - h2 / 2), 0.0f);
    float area_union = w1 * h1 + w2 * h2 - area_inter;
    return area_inter / area_union > IOU_THRESHOLD;
}

// uint16_t PostProcess::post_process(float *input_buffer, float *output_buffer, uint16_t *num_detections)
// {
//     auto start = std::chrono::high_resolution_clock::now();
//     dim3 threads_pre_block(48, 2);
//     dim3 blocks(175);
//     transform_results<<<blocks, threads_pre_block>>>(input_buffer, this->transformed);
//     check_status(cudaDeviceSynchronize());
//     auto end = std::chrono::high_resolution_clock::now();
//     auto diff = end - start;
//     std::cout << "Time taken by 1" << ": " << diff.count() << " seconds" << std::endl;
//     start = std::chrono::high_resolution_clock::now();
//     thrust::sequence(this->d_indices, this->d_indices + FEATURE_MAP_SIZE);
//     end = std::chrono::high_resolution_clock::now();
//     diff = end - start;
//     std::cout << "Time taken by 2" << ": " << diff.count() << " seconds" << std::endl;
//     start = std::chrono::high_resolution_clock::now();
//     thrust::sort(this->d_indices, this->d_indices + FEATURE_MAP_SIZE, [d_transformed = this->d_transformed] __device__(int a, int b)
//                  { return d_transformed[a * 16 + 4] > d_transformed[b * 16 + 4]; });
//     end = std::chrono::high_resolution_clock::now();
//     diff = end - start;
//     std::cout << "Time taken by 3" << ": " << diff.count() << " seconds" << std::endl;
//     start = std::chrono::high_resolution_clock::now();
//     check_status(cudaMemcpy(this->host_indices, this->indices, FEATURE_MAP_SIZE * sizeof(int), cudaMemcpyDeviceToHost));
//     check_status(cudaMemcpy(this->host_transformed, this->transformed, FEATURE_MAP_SIZE * 16 * sizeof(float), cudaMemcpyDeviceToHost));
//     end = std::chrono::high_resolution_clock::now();
//     diff = end - start;
//     std::cout << "Time taken by 4" << ": " << diff.count() << " seconds" << std::endl;
//     *num_detections = (uint16_t)MAX_DETECT;
//     start = std::chrono::high_resolution_clock::now();
//     int last = FEATURE_MAP_SIZE;
//     for (int i = 0; i < FEATURE_MAP_SIZE; ++i)
//     {
//         if (this->host_transformed[i * 16 + 4] < CONF_THRESHOLD)
//         {
//             last = i;
//             break;
//         }
//     }
//     for (int i = 0, j = 0; i < MAX_DETECT && j != -1; ++i)
//     {
//         int idx = this->host_indices[j];
//         if (this->host_transformed[idx * 16 + 4] < CONF_THRESHOLD)
//         {
//             *num_detections = (uint16_t)i;
//             break;
//         }
//         for (int item = 0; item < 16; ++item)
//         {
//             output_buffer[i * 16 + item] = this->host_transformed[idx * 16 + item];
//         }
//         int next = -1;
//         float *box = this->host_transformed + idx * 16;
//         for (; j < last; ++j)
//         {
//             int idx1 = this->host_indices[j];
//             if (idx1 == -1)
//             {
//                 continue;
//             }
//             if (check_iou(box, this->host_transformed + idx1 * 16))
//             {
//                 this->host_indices[j] = -1;
//             }
//             else if (next == -1)
//             {
//                 next = j;
//             }
//         }
//         j = next;
//     }
//     end = std::chrono::high_resolution_clock::now();
//     diff = end - start;
//     std::cout << "Time taken by 5" << ": " << diff.count() << " seconds" << std::endl;
//     return (uint16_t)cudaSuccess;
// }

// input buffer (1, 5, FEATURE_MAP_SIZE)  (DEVICE)
// output buffer (MAX_DETECTION, 5)  (HOST)
// 5: 4(xywh) + 1(score)
uint16_t PostProcess::post_process(float *input_buffer, float *output_buffer, uint16_t *num_detections)
{
    dim3 threads_per_block(48);
    dim3 blocks((FEATURE_MAP_SIZE + 47) / 48);
    // (1, 5, FEATURE_MAP_SIZE)
    transform_results<<<blocks, threads_per_block>>>(input_buffer, this->transformed, FEATURE_MAP_SIZE);
    // (1, FEATURE_MAP_SIZE, 5)

    check_status(cudaDeviceSynchronize());
    thrust::sequence(this->d_indices, this->d_indices + FEATURE_MAP_SIZE);
    thrust::sort(this->d_indices, this->d_indices + FEATURE_MAP_SIZE, [d_transformed = this->d_transformed] __device__(int a, int b)
                 { return d_transformed[a * 5 + 4] > d_transformed[b * 5 + 4]; });

    check_status(cudaMemcpy(this->host_indices, this->indices, FEATURE_MAP_SIZE * sizeof(int), cudaMemcpyDeviceToHost));
    check_status(cudaMemcpy(this->host_transformed, this->transformed, FEATURE_MAP_SIZE * 16 * sizeof(float), cudaMemcpyDeviceToHost));

    int end = FEATURE_MAP_SIZE;
    for (int i = 0; i < FEATURE_MAP_SIZE; ++i)
    {
        int idx = this->host_indices[i];
        if (this->host_transformed[idx * 5 + 4] < CONF_THRESHOLD)
        {
            end = i;
            break;
        }
    }

    if (end == 0)
    {
        *num_detections = 0;
        return (uint16_t)cudaSuccess;
    }

    int i = 0;
    for (int j = 0; i < MAX_DETECT && j != -1; ++i)
    {
        int idx = this->host_indices[j];
        for (int item = 0; item < 16; ++item)
        {
            output_buffer[i * 16 + item] = this->host_transformed[idx * 16 + item];
        }

        int next = -1;
        float *box = this->host_transformed + idx * 16;
        for (; j < end; ++j)
        {
            int idx1 = this->host_indices[j];
            if (idx1 == -1)
            {
                continue;
            }
            if (check_iou(box, this->host_transformed + idx1 * 16))
            {
                this->host_indices[j] = -1;
            }
            else if (next == -1)
            {
                next = j;
            }
        }
        j = next;
    }
    *num_detections = (uint16_t)i;
    return (uint16_t)cudaSuccess;
}

PostProcess *POSTPROCESS;

uint16_t postprocess_init_default()
{
    POSTPROCESS = new PostProcess();
    check_status(POSTPROCESS->init());
    return (uint16_t)cudaSuccess;
}

uint16_t postprocess_init(uint16_t max_detect, float conf_threshold, float iou_threshold, uint16_t feature_map_size)
{
    POSTPROCESS = new PostProcess(max_detect, conf_threshold, iou_threshold, feature_map_size);
    check_status(POSTPROCESS->init());
    return (uint16_t)cudaSuccess;
}

// input buffer (1, 32, FEATURE_MAP_SIZE)
// output buffer (MAX_DETECTION, 16)
// 16: 4(xywh) + 1(score) + 1(cls) + 10(kpnt)
uint16_t postprocess(float *input_buffer, float *output_buffer, uint16_t *num_detections)
{
    check_status(POSTPROCESS->post_process(input_buffer, output_buffer, num_detections));
    return (uint16_t)cudaSuccess;
}

uint16_t postprocess_destroy()
{
    check_status(POSTPROCESS->uninit());
    delete POSTPROCESS;
    return (uint16_t)cudaSuccess;
}

// input tensor shape (1, 5, FEATURE_MAP_SIZE)
// 5: 4(xywh) + 1(class)
// output shape (1, FEATURE_MAP_SIZE, 5)
__global__ void transform_results2(float *input_buffer, float *output_buffer, uint16_t FEATURE_MAP_SIZE)
{
    int x = blockIdx.x * blockDim.x + threadIdx.x;

    if (x < FEATURE_MAP_SIZE)
    {
        float max_cls = input_buffer[4 * FEATURE_MAP_SIZE + x];
        int cls = 0;
        for (int i = 5; i < 16; i++)
        {
            float tmp = input_buffer[i * FEATURE_MAP_SIZE + x];
            if (max_cls < tmp)
            {
                max_cls = tmp;
                cls = i - 4;
            }
        }
        output_buffer[x * 2] = max_cls;
        output_buffer[x * 2 + 1] = (float)cls;
    }
}

uint16_t postprocess_classify(float *input_buffer, uint16_t feature_map_size, uint16_t *cls)
{
    static float *TRANSFORMED = nullptr;
    static int *INDICES = nullptr;
    
    if (TRANSFORMED == nullptr)
    {
        check_status(cudaMalloc(&TRANSFORMED, FEATURE_MAP_SIZE * 2 * sizeof(float)));
        check_status(cudaMalloc(&INDICES, FEATURE_MAP_SIZE * sizeof(int)));
    }

    d_TRANSFORMED = thrust::device_ptr<float>(TRANSFORMED);
    d_INDICES = thrust::device_ptr<int>(INDICES);

    dim3 threads_per_block(48);
    dim3 blocks((feature_map_size + 47) / 48);

    transform_results2<<<blocks, threads_per_block>>>(input_buffer, TRANSFORMED, FEATURE_MAP_SIZE);

    check_status(cudaDeviceSynchronize());

    thrust::sequence(this->d_INDICES, this->d_INDICES + FEATURE_MAP_SIZE);
    thrust::sort(this->d_INDICES, this->d_INDICES + FEATURE_MAP_SIZE, [d_TRANSFORMED] __device__(int a, int b)
                 { return d_TRANSFORMED[a * 2] > d_TRANSFORMED[b * 2]; });

    int idx = 0;
    check_status(cudaMemcpy(&idx, INDICES, sizeof(int), cudaMemcpyDeviceToHost));
    float cls_f = 0;
    check_status(cudaMemcpy(cls_f, TRANSFORMED + 2 * sizeof(float) * idx + 1, sizeof(float), cudaMemcpyDeviceToHost));
    *cls = (uint16_t)cls_f;
}
