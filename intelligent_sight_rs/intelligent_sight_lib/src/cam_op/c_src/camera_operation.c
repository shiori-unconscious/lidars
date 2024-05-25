#include "mv_cam.h"
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

uint8_t CAMERA_NUMBER = 0;
tSdkCameraDevInfo *CAMERA_LIST;
CameraHandle *CAMERA_HANDLERS;
tSdkCameraCapbility *CAPABILITY_LIST;

uint8_t initialize_camera(uint8_t wanted_cam_number, uint32_t *image_width, uint32_t *image_height, uint8_t *already_initialized, uint32_t exposure_time)
{
    CameraSdkStatus status;

    check_status(CameraSdkInit(0));

    CAMERA_LIST = (tSdkCameraDevInfo *)malloc(sizeof(tSdkCameraDevInfo) * wanted_cam_number);

    INT camera_counts = wanted_cam_number;

    check_status(CameraEnumerateDevice(CAMERA_LIST, &camera_counts));

    printf("SDK: Camera count: %d\n", camera_counts);

    if (camera_counts < wanted_cam_number)
    {
        return CAMERA_STATUS_NUMBER_FEWER;
    }

    tSdkImageResolution resolution = {0};
    resolution.iIndex = 0;

    CAMERA_HANDLERS = (CameraHandle *)malloc(sizeof(CameraHandle) * wanted_cam_number);
    CAPABILITY_LIST = (tSdkCameraCapbility *)malloc(sizeof(tSdkCameraCapbility) * wanted_cam_number);

    CAMERA_NUMBER = 0;

    for (int i = 0; i < wanted_cam_number; i++)
    {
        printf("SDK: Camera %d: %s\n", i, CAMERA_LIST[i].acFriendlyName);

        check_status(CameraInit(&CAMERA_LIST[i], -1, -1, &CAMERA_HANDLERS[i]));
        printf("SDK: Finish CameraInit\n");
        CAMERA_NUMBER++;
        *already_initialized = 1;

        check_status_retry(CameraGetCapability(CAMERA_HANDLERS[i], &CAPABILITY_LIST[i]));
        printf("SDK: Finish CameraGetCapability\n");

        check_status_retry(CameraSetAeState(CAMERA_HANDLERS[i], FALSE));
        printf("SDK: Finish setting manual exposure CameraSetAeState\n");

        // double exposure_line_time;
        // check_status(CameraGetExposureLineTime(CAMERA_HANDLERS[i], &exposure_line_time));

        // double from_value = CAPABILITY_LIST[i].sExposeDesc.uiExposeTimeMin * exposure_line_time;
        // double to_value = CAPABILITY_LIST[i].sExposeDesc.uiExposeTimeMax * exposure_line_time;
        // printf("Exposure time range: %f - %f\n", from_value, to_value);
        // printf("Current exposure line time: %f\n", exposure_line_time);

        check_status_retry(CameraSetExposureTime(CAMERA_HANDLERS[i], (double)exposure_time));

        double exposure_time_feedback;
        check_status(CameraGetExposureTime(CAMERA_HANDLERS[i], &exposure_time_feedback));
        printf("SDK: Finish setting exposure time %lf us\n", exposure_time_feedback);
        // printf("Current exposure time: %f\n", exposure_time);

        check_status_retry(CameraSetIspOutFormat(CAMERA_HANDLERS[i], CAMERA_MEDIA_TYPE_RGB8));
        printf("SDK: Finish setting output format RGB8\n");

        check_status_retry(CameraSetImageResolution(CAMERA_HANDLERS[i], &resolution));

        tSdkImageResolution resolution_afterward = {0};
        resolution_afterward.iIndex = 1;
        CameraGetImageResolution(CAMERA_HANDLERS[i], &resolution_afterward);
        // printf("%s\n", resolution_afterward.acDescription[resolution_afterward.iIndex]);

        image_height[i] = resolution_afterward.iHeight;
        image_width[i] = resolution_afterward.iWidth;

        check_status_retry(CameraPlay(CAMERA_HANDLERS[i]));
        printf("SDK: Camera is working now\n");
    }

    return CAMERA_STATUS_SUCCESS;
}

uint8_t get_image(uint8_t camera_index, BYTE *image_data, uint32_t *image_width, uint32_t *image_heigh, uint8_t flip_flag)
{
    CameraSdkStatus status;
    BYTE *pbyBuffer;
    tSdkFrameHead sFrameInfo;

    int iDisplayFrames = 0;
    int i = 0;

    if (camera_index >= CAMERA_NUMBER)
    {
        return CAMERA_STATUS_INDEX_EXCEEDING;
    }

    check_status(CameraGetImageBuffer(CAMERA_HANDLERS[camera_index], &sFrameInfo, &pbyBuffer, 100));
    if (sFrameInfo.iWidth < 0 || sFrameInfo.iHeight < 0)
    {
        return CAMERA_STATUS_ERROR_IMGSIZE;
    }

    check_status(CameraImageProcess(CAMERA_HANDLERS[camera_index], pbyBuffer, image_data, &sFrameInfo));

    check_status(CameraReleaseImageBuffer(CAMERA_HANDLERS[camera_index], pbyBuffer));

    if (sFrameInfo.uiMediaType != CAMERA_MEDIA_TYPE_RGB8)
    {
        printf("SDK: Found camera media type %d\n", sFrameInfo.uiMediaType);
        return CAMERA_STATUS_ERROR_TYPE;
    }

    if (flip_flag != 0)
    {
        check_status(CameraFlipFrameBuffer(image_data, &sFrameInfo, flip_flag));
    }

    *image_width = (uint32_t)sFrameInfo.iWidth;
    *image_heigh = (uint32_t)sFrameInfo.iHeight;

    return CAMERA_STATUS_SUCCESS;
}

uint8_t uninitialize_camera()
{
    for (int i = 0; i < CAMERA_NUMBER; i++)
    {
        check_status(CameraStop(CAMERA_HANDLERS[i]));
        check_status(CameraUnInit(CAMERA_HANDLERS[i]));
    }

    free(CAMERA_LIST);
    free(CAMERA_HANDLERS);
    free(CAPABILITY_LIST);

    return CAMERA_STATUS_SUCCESS;
}
