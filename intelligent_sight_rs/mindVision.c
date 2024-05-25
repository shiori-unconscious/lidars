#include <stdio.h>
#include <time.h>
#include "CameraConfig.h"
#define CAMERA_MAX_COUNT 6
struct timespec last;

// void callback(CameraHandle hCamera, BYTE *pFrameBuffer, tSdkFrameHead *pFrameInfo, PVOID pContext)
// {
//     struct timespec now;
//     clock_gettime(CLOCK_MONOTONIC, &now);
//     long elapsed = (now.tv_sec - last.tv_sec) * 1e9 + (now.tv_nsec - last.tv_nsec);
//     last = now;
//     printf("fps:%lf, received%d %d %d %d\n", 1e9 / elapsed, pFrameInfo->uBytes, pFrameInfo->uiMediaType, pFrameInfo->iWidth, pFrameInfo->iHeight);
// }

int main()
{
    CameraSdkInit(0);

    int iCameraCounts = CAMERA_MAX_COUNT;
    tSdkCameraDevInfo tCameraEnumList[CAMERA_MAX_COUNT];

    CameraEnumerateDevice(&tCameraEnumList[0], &iCameraCounts);

    printf("Camera count: %d\n", iCameraCounts);
    for (int i = 0; i < iCameraCounts; i++)
    {
        printf("Camera %d: %s\n", i, tCameraEnumList[i].acFriendlyName);
    }
    CameraHandle hCamera;
    CameraInit(&tCameraEnumList[0], -1, -1, &hCamera);
    CameraPlay(hCamera);
    clock_gettime(CLOCK_MONOTONIC, &last);
    tSdkCameraCapbility tCapability;
    CameraGetCapability(hCamera, &tCapability);
    CameraSetAeState(hCamera, FALSE);
    while (1)
    {
        BYTE *pbyBuffer;
        tSdkFrameHead sFrameInfo;
        CameraGetImageBuffer(hCamera, &sFrameInfo, &pbyBuffer, 1000);
        struct timespec now;
        clock_gettime(CLOCK_MONOTONIC, &now);
        long elapsed = (now.tv_sec - last.tv_sec) * 1e9 + (now.tv_nsec - last.tv_nsec);
        last = now;
        printf("fps:%lf, received%d %d %d %d\n", 1e9 / elapsed, sFrameInfo.uBytes, sFrameInfo.uiMediaType, sFrameInfo.iWidth, sFrameInfo.iHeight);
        CameraReleaseImageBuffer(hCamera, pbyBuffer);
    }
    printf("ending\n");
    CameraUnInit(hCamera);
    return 0;
}