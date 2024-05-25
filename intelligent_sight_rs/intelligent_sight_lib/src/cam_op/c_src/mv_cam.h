#ifndef MINDVISIONCAMERA_H

#define MINDVISIONCAMERA_H

#include <CameraApi.h>
#include <CameraDefine.h>
#include <CameraStatus.h>
#include <stdint.h>
#include <time.h>

#define CAMERA_MAX_COUNT 6
#define CAMERA_STATUS_INDEX_EXCEEDING 58
#define CAMERA_STATUS_NUMBER_FEWER 59
#define CAMERA_STATUS_ERROR_TYPE 60
#define CAMERA_STATUS_ERROR_IMGSIZE 61

#define delay(milli_seconds)                         \
    do                                               \
    {                                                \
        clock_t start_time = clock();                \
        while (clock() < start_time + milli_seconds) \
            ;                                        \
    } while (0)

#define check_status(fun)                        \
    do                                           \
    {                                            \
        int ret_status = (fun);                  \
        if (ret_status != CAMERA_STATUS_SUCCESS) \
        {                                        \
            return (uint8_t) - ret_status;       \
        }                                        \
    } while (0)

#define check_status_retry(fun)                      \
    do                                               \
    {                                                \
        int i = 0, ret_status = 0;                   \
        for (; i < 5; ++i)                           \
        {                                            \
            ret_status = (fun);                      \
            if (ret_status == CAMERA_STATUS_SUCCESS) \
            {                                        \
                break;                               \
            }                                        \
            delay(2);                                \
        }                                            \
        if (i == 10)                                 \
        {                                            \
            return (uint8_t) - ret_status;           \
        }                                            \
    } while (0)

uint8_t
initialize_camera(uint8_t wanted_cam_number,
                  uint32_t *image_width,
                  uint32_t *image_height,
                  uint8_t *already_initialized,
                  uint32_t exposure_time);

uint8_t
get_image(uint8_t camera_index,
          BYTE *image_data,
          uint32_t *image_width,
          uint32_t *image_heigh,
          uint8_t flip_flag);

uint8_t uninitialize_camera();

#endif