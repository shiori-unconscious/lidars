#include <stdio.h>
#include <time.h>
#include <CameraApi.h>
#define CAMERA_MAX_COUNT 6
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <stdlib.h>
#include <string.h>
struct timespec last;
BYTE *pbyBuffer;
tSdkFrameHead sFrameInfo;
GLuint width = 1280, height = 1024;
GLuint vertexShader, fragmentShader, shaderProgram;
GLuint VBO, VAO;
GLfloat vertices[] = {
    // 顶点坐标   纹理坐标
    -1.0f, -1.0f, 0.0f, 1.0f,
    1.0f, -1.0f, 1.0f, 1.0f,
    1.0f, 1.0f, 1.0f, 0.0f,
    -1.0f, 1.0f, 0.0f, 0.0f};

int vertexAttribLocation, texCoordAttribLocation, textureUniformLocation;

const char *vertexShaderSource =
    "#version 330 core\n"
    "layout (location = 0) in vec2 aPos;\n"
    "layout (location = 1) in vec2 aTexCoord;\n"
    "out vec2 TexCoord;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);\n"
    "   TexCoord = aTexCoord;\n"
    "}\0";

const char *fragmentShaderSource =
    "#version 330 core\n"
    "out vec4 FragColor;\n"
    "in vec2 TexCoord;\n"
    "uniform sampler2D texture1;\n"
    "void main()\n"
    "{\n"
    "   FragColor = texture(texture1, TexCoord);\n"
    "}\n\0";

void initShaderProgram()
{
    // 编译顶点着色器
    vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
    glCompileShader(vertexShader);
    // 编译片段着色器
    fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
    glCompileShader(fragmentShader);
    // 创建着色器程序
    shaderProgram = glCreateProgram();
    glAttachShader(shaderProgram, vertexShader);
    glAttachShader(shaderProgram, fragmentShader);
    glLinkProgram(shaderProgram);
    // 获取着色器中的属性和uniform位置
    vertexAttribLocation = glGetAttribLocation(shaderProgram, "aPos");
    texCoordAttribLocation = glGetAttribLocation(shaderProgram, "aTexCoord");
    textureUniformLocation = glGetUniformLocation(shaderProgram, "texture1");
}

void initBuffers()
{
    glGenVertexArrays(1, &VAO);
    glGenBuffers(1, &VBO);
    glBindVertexArray(VAO);
    glBindBuffer(GL_ARRAY_BUFFER, VBO);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
    // 顶点位置属性
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 4 * sizeof(GLfloat), (void *)0);
    glEnableVertexAttribArray(0);
    // 纹理坐标属性
    glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 4 * sizeof(GLfloat), (void *)(2 * sizeof(GLfloat)));
    glEnableVertexAttribArray(1);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindVertexArray(0);
}

void display(GLFWwindow *window, GLuint textureID)
{
    glClear(GL_COLOR_BUFFER_BIT);
    glUseProgram(shaderProgram);
    glBindTexture(GL_TEXTURE_2D, textureID);
    glBindVertexArray(VAO);
    glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    glBindVertexArray(0);
    glfwSwapBuffers(window);
    glfwPollEvents();
}

int main()
{
    if (!glfwInit())
    {
        exit(EXIT_FAILURE);
    }
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    GLFWwindow *window = glfwCreateWindow(width, height, "Captured", NULL, NULL);
    glfwMakeContextCurrent(window);
    if (glewInit() != GLEW_OK)
    {
        exit(EXIT_FAILURE);
    }
    glfwSwapInterval(1);

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

    clock_gettime(CLOCK_MONOTONIC, &last);
    tSdkCameraCapbility tCapability;
    CameraGetCapability(hCamera, &tCapability);
    size_t len = tCapability.sResolutionRange.iHeightMax * tCapability.sResolutionRange.iWidthMax * 3;
    BYTE *imgBuffer = (BYTE *)malloc(len);
    CameraSetAeState(hCamera, 0);
    CameraPlay(hCamera);

    GLuint textureID;
    glGenTextures(1, &textureID);
    glBindTexture(GL_TEXTURE_2D, textureID);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, NULL);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    initShaderProgram();
    initBuffers();
    while (!glfwWindowShouldClose(window))
    {
        if (CameraGetImageBuffer(hCamera, &sFrameInfo, &pbyBuffer, 1000) == CAMERA_STATUS_SUCCESS)
        {
            struct timespec now;
            clock_gettime(CLOCK_MONOTONIC, &now);
            long elapsed = (now.tv_sec - last.tv_sec) * 1e9 + (now.tv_nsec - last.tv_nsec);
            last = now;
            printf("fps:%lf, received%d %d %d %d\n", 1e9 / elapsed, sFrameInfo.uBytes, sFrameInfo.uiMediaType, sFrameInfo.iWidth, sFrameInfo.iHeight);
            CameraImageProcess(hCamera, pbyBuffer, imgBuffer, &sFrameInfo);
            CameraReleaseImageBuffer(hCamera, pbyBuffer);

            glBindTexture(GL_TEXTURE_2D, textureID);
            glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, width, height, GL_RGB, GL_UNSIGNED_BYTE, imgBuffer);

            display(window, textureID);
        }
        else
        {
            printf("error no image fetched\n");
            break;
        }
    }
    free(imgBuffer);
    printf("ending\n");
    CameraUnInit(hCamera);
    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}