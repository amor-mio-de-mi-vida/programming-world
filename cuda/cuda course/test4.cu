#include <opencv2/opencv.hpp>
#include <iostream>

using namespace std;
using namespace cv;

// Sobel:
// kernel   x00 x01 x02      m00 m01 m02 m03 m04        r00 r01 r02 r03 r04
//          x10 x11 x12      m10 m11 m12 m13 m14        r10 r11 r12 r13 r14
//          x20 x21 x22      m20 m21 m22 m23 m24        r20 r21 r22 r23 r24
//                           m30 m31 m32 m33 m34        r30 r31 r32 r33 r34
//                           m40 m41 m42 m43 m44        r40 r41 r42 r43 r44
//
//  r11 = x00*m00 + x01*m01 + x02*m02 + x10*m10 + x11*m11 + x12*m12 + x20*m20 + x21*m21 + x22*m22
//  Gx  1 0 -1      Gy   1  2  1
//      2 0 -2           0  0  0
//      1 0 -1          -1 -2 -1

__global__ void sobel_gpu(unsigned char* in, unsigned char* out, const int Height, constint Width);

int main() {
    Mat img = imread("1.jpg", 0);
    int height = img.rows;
    int width = img.cols;

    Mat gaussImg;
    GaussianBlur(img, gaussImg, Size(3,3), 0, 0, BORDER_DEFAULT);

    Mat dst_gpu(height, width, CV_8UC1, Scalar(0));

    int memsize = height * width * sizeof(unsigned char);

    unsigned char* in_gpu;
    unsigned char* out_gpu;

    cudaMalloc((void**) &in_gpu, memsize);
    cudaMalloc((void**) &out_gpu, memsize);

    dim3 threadsPerBlock(32, 32);
    dim3 blocksPerGrid((width + threadPerBlock.x - 1) / threadPerBlock.x , (height + threadPerBlock.y -1) / threadPerBlock.y);

    cudaMemcpy(in_gpu, gaussImg.data, memsize, cudaMemcpyHostToDevice);

    sobel_gpu<<<blocksPerGrid, threadsPerBlock>>>(in_gpu, out_gpu, height, width);

    cudaMemcpy(dst_gpu.data, out_gpu, memsize, cudaMemcpyDeviceToHost);

    imwrite("save.png", dst_gpu);
    cudaFree(in_gpu);
    cudaFree(out_gpu);

    return0;
}

__global__ void sobel_gpu(unsigned char* in, unsigned char* out, const int Height, constint Width) {
    int x = blockDim.x * blockIdx.x + threadIdx.x;
    int y = blockDim.y * blockIdx.y + threadIdx.y;

    int index = y * Width + x;

    int Gx = 0;
    int Gy = 0;
    unsigned char x0, x1, x2, x3, x4, x5, x6, x7, x8;
    // SM register
    if (x > 0 && x < (Width - 1) && y > 0 && y < (Height - 1)) {
        x0 = in[(y-1) * Width + (x-1)];
        x1 = in[(y-1) * Width + x];
        x2 = in[(y-1) * Width + (x+1)];
        x3 = in[(y) * Width + (x-1)];
        x4 = in[(y) * Width + x];
        x5 = in[(y) * Width + (x+1)];
        x6 = in[(y+1) * Width + (x-1)];
        x7 = in[(y+1) * Width + x];
        x8 = in[(y+1) * Width + (x+1)];

        Gx = (x0 + 2*x3 + x6) - (x2 + 2*x5 + x7);
        Gy = (x0 + 2*x1 + x2) - (x6 + 2*x7 + x8);
        
        out[index] = (Gx + Gy) / 2;
    }

}