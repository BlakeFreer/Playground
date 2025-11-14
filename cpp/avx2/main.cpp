#include <immintrin.h>

#include <iostream>

void print(__m256d x) {
    std::cout << "[";
    alignas(32) double items[4];
    _mm256_store_pd(items, x);
    for (int i = 0; i < 4; i++) {
        if (i > 0) {
            std::cout << ", ";
        }
        std::cout << items[i];
    }
    std::cout << "]" << std::endl;
}

int main(void) {
    __m256d A = _mm256_set_pd(1, 2, 3, 4);
    __m256d B = _mm256_set_pd(10, 100, 1000, 10000);
    __m256d C = _mm256_mul_pd(A, B);

    print(C);

    __m128d lo = _mm256_extractf128_pd(C, 0);
    __m128d hi = _mm256_extractf128_pd(C, 1);
    __m128d s1 = _mm_add_pd(lo, hi);
    __m128d s2 = _mm_shuffle_pd(s1, s1, 0b01);
    __m128d s3 = _mm_add_pd(s1, s2);

    double sum = _mm_cvtsd_f64(s3);

    std::cout << sum << std::endl;

    return 0;
}
