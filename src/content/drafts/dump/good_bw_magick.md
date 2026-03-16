+++
title = "convert images to black and white with imagemagick"
date = 2020-09-07
draft = true
[extra]
categories = ["💡", "📚"]
+++

> convert \<input\> -monochrome \<output\>

![jpeg](/public/img/magick_test_files/magick_test_4_0.jpg)
no

> convert \<input\> -colorspace Gray \<output\>

![jpeg](/public/img/magick_test_files/magick_test_6_0.jpg)
yes

```
!curl -O https://live.staticflickr.com/65535/49454964733_d01dda2224_k_d.jpg
```

      % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                     Dload  Upload   Total   Spent    Left  Speed
    100  364k    0  364k    0     0  3193k      0 --:--:-- --:--:-- --:--:-- 3193k

```
!ls
```

    49454964733_d01dda2224_k_d.jpg	sample_data

```
!sudo apt install imagemagick

!convert --version
```

```
from IPython.display import Image

Image("49454964733_d01dda2224_k_d.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_3_0.jpg)

```
!convert 49454964733_d01dda2224_k_d.jpg -monochrome true_bw.jpg



Image("true_bw.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_4_0.jpg)

```
!convert 49454964733_d01dda2224_k_d.jpg -remap pattern:gray50 remap.jpg



Image("remap.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_5_0.jpg)

```
!convert 49454964733_d01dda2224_k_d.jpg -colorspace Gray true_grey.jpg



Image("true_grey.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_6_0.jpg)

```
!convert 49454964733_d01dda2224_k_d.jpg -separate seperate.jpg



Image("seperate-0.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_7_0.jpg)

```
Image("seperate-1.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_8_0.jpg)

```
Image("seperate-2.jpg")
```

![jpeg](/public/img/magick_test_files/magick_test_9_0.jpg)

```

```
