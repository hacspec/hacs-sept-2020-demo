use hacspec_lib::prelude::*;

use poly_demo::*;

#[test]
fn kat_ntru_prime_653() {
    let m: Seq<i128> = Seq::from_native_slice(&[
        -1, -1, 0, -1, 0, 0, 0, -1, -1, 0, 0, 0, -1, 0, -1, 0, -1, 1, 0, -1, 0, 0, 0, -1, -1, 0,
        -1, 0, 0, 1, 0, 0, 0, -1, -1, 0, 1, -1, 0, -1, -1, 0, 0, -1, -1, 0, 0, 0, 0, 0, -1, 0, 1,
        0, -1, 1, -1, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, -1, -1, -1, 0, -1, -1, 0, -1, -1, -1, -1, -1,
        0, 0, 0, 0, 0, -1, -1, -1, 0, 0, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, -1, -1, -1, -1, 0,
        0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, 0, -1, 0,
        -1, 0, 0, 0, 1, -1, -1, 0, -1, -1, 0, -1, -1, 0, 0, 0, 0, 0, -1, 1, 0, -1, 0, 0, 0, -1, 1,
        0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, 0, 0, 0,
        0, 0, 1, -1, 0, 1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, 0, 1, -1, 1, 0, 0, 0, -1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1, -1,
        0, 0, -1, 0, -1, -1, -1, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, -1, -1, 0, 0,
        0, 0, 0, 0, -1, 0, -1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 1, 0, 0, 0, 0,
        -1, 0, 0, 0, 0, -1, 0, -1, -1, 0, -1, -1, -1, 0, 0, -1, -1, -1, 0, -1, 0, -1, 0, 0, -1, 0,
        0, 0, -1, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, -1, -1, 0, 0, 0, 1,
        0, -1, 0, 1, -1, -1, 0, 0, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 1, -1, 1, -1, 0, 0, -1, 0,
        -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, -1, -1, 0, 1, 0, 0, 0, -1, 0, 1, 0, 0, 0, 1, 0, 0,
        0, -1, 0, 0, 0, -1, -1, 0, 0, 1, 0, 0, 1, 0, 1, -1, -1, 0, 0, -1, 0, 0, 0, 0, -1, -1, -1,
        0, 0, 0, 0, 1, 0, 0, -1, 1, 0, 0, 0, 0, 0, -1, -1, -1, 0, -1, 1, -1, -1, 0, 0, 0, -1, 0, 0,
        1, 1, -1, 0, 1, -1, -1, 0, -1, 0, 0, 0, 0, -1, -1, -1, -1, 0, 0, 0, -1, -1, 1, 1, 1, 1, 0,
        1, 1, 0, 1, 0, 0, 0, -1, 0, 1, 0, 0, -1, 0, 0, 0, -1, 0, 1, -1, -1, 0, 0, 0, -1, 1, -1, 0,
        0, 0, 1, 0, -1, 0, 1, 1, 1, 0, 0, 1, -1, -1, 1, 0, -1, 0, -1, 1, -1, 0, 0, 1, 0, 0, 0, -1,
        0, 0, 1, 1, 1, -1, -1, 0, 0, -1, -1, 0, 0, -1, 1, 1, -1, 0, 0, 0, -1, 1, -1, -1, 0, -1, -1,
        -1, 0, 1, -1, 1, 0, 0, 1, 1, -1, 0, 0, -1, 1, 0, -1, 0, 0, -1, 0, 1, 0, 1, 0, 0, 0, 1, 0,
        1, 0, -1, -1, -1, -1, 0, 0, -1, 0, 0, -1, -1, 0, 0, -1, -1, 0, 1, -1, 0, 0, -1, -1, -1, 0,
        0, 1, 1, 0, 1, 0, 1, 0, 0, 0, -1, -1, 0, -1, -1, 1, 0, -1, -1, 1, -1, -1, 0, 0, 0, 0,
    ]);
    let pk: Seq<i128> = Seq::from_native_slice(&[
        380, 284, 2724, 4570, 1799, 2022, 4248, 4208, 476, 790, 1171, 2999, 4381, 3380, 1, 1013,
        3606, 4296, 3732, 2077, 4415, 3558, 1931, 2983, 1589, 3234, 756, 3105, 3554, 133, 1210,
        4599, 4557, 2373, 413, 391, 1674, 2254, 2714, 2343, 825, 710, 836, 2436, 4039, 4209, 4614,
        3850, 734, 2593, 242, 1887, 525, 1147, 2993, 2591, 4613, 4130, 2417, 3830, 625, 1662, 4050,
        3114, 465, 420, 105, 2627, 3094, 2060, 1831, 2933, 946, 1761, 2854, 1365, 2621, 3473, 566,
        3361, 39, 1863, 2140, 4210, 2446, 2968, 1998, 4364, 3280, 1889, 3394, 2917, 2865, 2347,
        2971, 2325, 1205, 1893, 2646, 3010, 3399, 2258, 137, 4519, 2754, 1965, 2530, 1701, 2199,
        1383, 1232, 650, 4374, 239, 2267, 3010, 4565, 3537, 1076, 230, 4449, 844, 1120, 2322, 743,
        3998, 2758, 1627, 2641, 3381, 2388, 1616, 1510, 4442, 3665, 1226, 1682, 1906, 1278, 234,
        1953, 2192, 3396, 503, 3330, 1829, 1970, 2392, 3841, 3202, 529, 1332, 1701, 400, 2915, 839,
        4617, 1476, 1188, 3598, 1484, 2589, 1994, 2224, 3665, 3336, 1031, 955, 3301, 3434, 2024,
        1174, 666, 3650, 270, 1495, 3982, 840, 4312, 2455, 284, 4481, 2283, 4009, 440, 2994, 3855,
        2676, 3626, 666, 3291, 3859, 2207, 1404, 4450, 3405, 440, 2896, 3414, 252, 3966, 2333,
        3430, 3760, 1556, 1304, 2606, 405, 1052, 3918, 3718, 419, 4379, 3367, 3245, 2471, 3595,
        2836, 78, 3935, 2072, 2690, 1156, 1447, 4269, 2950, 3280, 4096, 4231, 3705, 1582, 1571,
        1307, 1931, 3385, 2406, 914, 2686, 499, 2590, 1675, 1466, 1811, 3982, 2787, 2852, 1181,
        2668, 381, 3492, 2444, 2343, 237, 1724, 1680, 1884, 1968, 1163, 3804, 4612, 3694, 247,
        2798, 497, 3458, 3127, 1471, 2641, 1487, 662, 3299, 1657, 3075, 2671, 3407, 1524, 1642,
        1231, 2708, 2208, 3199, 3788, 2247, 4568, 2489, 842, 3363, 1232, 1953, 1270, 3637, 3298,
        1544, 1952, 988, 1055, 3038, 3892, 1516, 3470, 3530, 2226, 4009, 4029, 3703, 1986, 2340,
        4469, 3408, 975, 3330, 719, 1376, 3685, 1030, 3155, 2364, 2794, 1455, 3648, 3428, 2627,
        4472, 1950, 1291, 670, 1377, 4516, 494, 678, 1157, 4408, 2108, 1202, 1234, 2021, 3352,
        4365, 88, 3653, 4220, 1033, 818, 1917, 1602, 1159, 4601, 1567, 1614, 816, 4085, 675, 3598,
        3170, 290, 787, 1694, 2946, 2953, 2519, 688, 3688, 1829, 3014, 709, 1528, 4092, 2168, 1303,
        556, 222, 1079, 3380, 185, 1282, 1794, 1581, 2566, 129, 3936, 3891, 292, 3150, 429, 4093,
        1582, 3414, 915, 55, 126, 3879, 240, 2443, 284, 3877, 4507, 1418, 1497, 3820, 113, 2453,
        4082, 2752, 4465, 3008, 1190, 31, 1959, 3574, 2502, 1402, 1994, 3164, 1842, 2919, 4430,
        458, 2297, 290, 390, 2753, 4163, 2217, 2334, 867, 463, 4309, 2186, 159, 2383, 2243, 1585,
        4551, 4126, 2707, 3376, 1436, 2207, 1900, 1913, 3419, 883, 1167, 1877, 4503, 1465, 2327,
        1516, 578, 1216, 274, 365, 2536, 928, 2977, 1649, 4455, 2309, 3812, 1442, 4386, 1937, 4307,
        2866, 1849, 4157, 3587, 1319, 2430, 4101, 797, 122, 2027, 3586, 4291, 2491, 2150, 933,
        3001, 2230, 186, 3894, 1308, 3574, 1688, 2290, 768, 2127, 3893, 4505, 960, 2466, 4209,
        4058, 3961, 1919, 494, 787, 4486, 1498, 143, 3316, 3790, 4319, 3069, 3199, 4146, 176, 1630,
        3328, 755, 644, 3927, 485, 394, 3021, 1622, 598, 390, 2875, 2177, 3021, 4441, 1398, 3039,
        2606, 3890, 2907, 456, 2283, 4161, 2651, 1732, 2719, 4552, 4182, 392, 1276, 501, 3212,
        2145, 1958, 3038, 3802, 2360, 2208, 1541, 2638, 2800, 3285, 3205, 804, 4326, 2448, 2134,
        3595, 1406, 3405, 3090, 2777, 3407, 180, 973, 3321, 4324, 4503, 1172, 1267, 2135, 2910,
        2156, 705, 3723, 855, 3168, 3836, 1164, 2240, 187, 2460, 301, 3553, 4183, 4014, 1246, 1546,
        3250, 1306, 4236, 172, 4163, 1123, 4547, 3705, 1581, 149, 4349, 1480, 4501, 820, 367, 1760,
        2078, 3619, 218, 2232, 2223, 2260, 1254, 2699, 3409, 254, 487, 2074, 2927, 3017, 4209,
        3944, 2167, 1905, 1180, 3793, 95, 792, 2453, 2418, 2662, 4292, 2056, 1313, 2978, 603, 979,
        3085, 2046, 1196, 1793, 2855, 4536, 493, 3911, 3531, 551, 3679, 3312, 2902, 4513, 1283,
        1555, 4527, 2331, 2761, 2088, 0,
    ]);
    let sk: (Seq<i128>, Seq<i128>) = (
        Seq::from_native_slice(&[
            0, -1, 1, -1, 0, 0, -1, 1, -1, 0, -1, 0, -1, -1, 0, 0, -1, -1, 0, 1, 0, 0, -1, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1, 1, -1, 0, 0, 1, -1, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, -1, -1,
            0, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, -1, -1,
            -1, 0, -1, -1, -1, -1, -1, 0, 0, 0, -1, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1,
            0, 0, 0, 0, -1, -1, 0, 0, -1, 0, -1, -1, 1, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 0, -1,
            0, 1, 0, 0, 0, -1, -1, 0, 0, 1, 1, -1, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, -1, 1, -1, -1, 1,
            0, 0, -1, 0, -1, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, 1, 0, 0, -1, 0, 0, 1, -1, -1, 0,
            1, 0, 0, 0, -1, -1, -1, -1, 0, 0, -1, -1, 0, -1, 0, -1, -1, -1, 0, 1, 0, 0, 0, 0, -1,
            0, 0, 0, 0, -1, 0, 0, -1, -1, -1, -1, 0, 1, -1, 0, -1, -1, -1, -1, -1, -1, -1, 0, 0,
            -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, -1,
            0, -1, -1, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, -1, 0, -1, -1, 0, 0, 0, 0, -1, -1, -1,
            0, -1, -1, 0, -1, 0, 0, 0, -1, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, -1, -1, 0, 0, 0, 0,
            0, -1, 0, 0, 0, -1, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 1, 0, 0, 0, -1, -1, 0, 0,
            -1, -1, 0, -1, 0, 0, 1, -1, -1, 1, 1, 0, -1, 1, 0, 1, 0, 1, 0, 0, -1, -1, 0, 1, 0, 1,
            -1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 1, 1, -1, -1,
            -1, 0, -1, 0, -1, 0, 0, 0, 0, -1, -1, 1, 0, 1, 0, 0, -1, -1, -1, -1, 0, 1, 1, 0, 0, 0,
            -1, 0, -1, -1, 0, 0, -1, -1, 0, -1, 0, 1, -1, 0, 0, 0, -1, 0, -1, 0, 0, 0, 1, 0, 0, 1,
            -1, 0, 0, 0, 0, -1, 0, -1, 0, 0, 0, -1, 0, 0, 1, 0, -1, 0, -1, 1, 1, -1, 0, 0, -1, 0,
            -1, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, -1, -1, 0, 0, 0, -1, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 1, -1, -1, -1, -1, -1, 0, 0, -1, -1, -1, 0, -1, 0, 0, 1, -1, -1, 0, -1, 0, 0, 0,
            0, 1, -1, 0, -1, -1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, -1, 0, 1, 0, 1, 0, 0, -1, 0, 0,
            1, 0, -1, 0, -1, 0, 0, 0, 1, -1, 1, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0,
            -1, 0, -1, -1, 0, 0, 0, -1, 0, 1, 0, 0, 0, 1, 0, -1, -1, 0, 0, -1, 0, 1, 0, 0, -1, 1,
            0, -1, 1, 0, 0, -1, -1, 0, -1, -1, 1, 0, -1, 1, 0, 0, 0, -1, -1, 0, -1, 0, 0, 0, 0, 0,
            0, -1, 0,
        ]),
        Seq::from_native_slice(&[
            2, 2, 1, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 0, 0, 1, 1, 1, 2, 0, 1, 2, 2, 1, 1, 0, 1, 0, 1,
            0, 2, 2, 0, 2, 1, 1, 2, 0, 1, 0, 0, 0, 1, 0, 2, 1, 2, 2, 0, 0, 0, 0, 2, 1, 0, 0, 1, 0,
            2, 0, 2, 2, 0, 2, 0, 1, 0, 1, 2, 2, 2, 0, 0, 2, 1, 1, 0, 1, 0, 2, 0, 2, 1, 0, 1, 1, 1,
            2, 2, 1, 1, 1, 2, 1, 2, 1, 1, 2, 0, 2, 0, 0, 1, 0, 0, 0, 2, 1, 1, 2, 2, 2, 2, 2, 0, 0,
            1, 1, 2, 1, 1, 0, 0, 2, 1, 1, 0, 0, 1, 1, 2, 1, 2, 0, 0, 0, 0, 2, 2, 2, 0, 1, 2, 2, 2,
            2, 2, 0, 0, 2, 0, 0, 1, 2, 0, 2, 2, 1, 2, 2, 0, 1, 1, 2, 2, 2, 1, 0, 1, 0, 2, 0, 0, 2,
            0, 0, 2, 0, 1, 1, 2, 2, 1, 2, 1, 2, 2, 0, 2, 2, 2, 2, 1, 0, 1, 1, 0, 0, 1, 2, 1, 1, 1,
            1, 0, 1, 2, 2, 0, 2, 1, 2, 2, 0, 1, 0, 2, 1, 0, 0, 2, 2, 2, 2, 1, 1, 0, 1, 1, 2, 2, 0,
            0, 0, 2, 0, 0, 2, 2, 2, 2, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 2, 0, 1, 1, 0, 0, 1, 1, 1, 2,
            1, 2, 0, 2, 0, 1, 0, 1, 1, 1, 1, 2, 2, 2, 1, 0, 0, 0, 2, 2, 0, 1, 0, 2, 2, 1, 0, 2, 1,
            1, 1, 1, 1, 2, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 2, 0, 1, 2, 2, 0, 1, 1, 0, 1, 2, 2, 0,
            2, 0, 0, 1, 0, 1, 0, 0, 1, 2, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 2, 2, 1, 1, 1, 0, 1,
            0, 0, 1, 2, 1, 1, 2, 1, 1, 2, 1, 0, 2, 2, 0, 1, 2, 2, 1, 1, 1, 2, 2, 1, 0, 0, 0, 0, 0,
            0, 1, 1, 0, 2, 2, 0, 0, 2, 2, 1, 1, 1, 2, 2, 2, 1, 2, 0, 1, 2, 2, 0, 1, 0, 2, 0, 0, 1,
            1, 0, 1, 2, 1, 2, 0, 0, 2, 2, 1, 2, 2, 2, 0, 2, 0, 0, 1, 1, 0, 2, 0, 1, 0, 1, 2, 2, 2,
            2, 2, 1, 0, 1, 1, 2, 0, 1, 1, 2, 0, 0, 1, 0, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 1, 1, 0,
            1, 0, 0, 0, 2, 1, 0, 0, 1, 2, 0, 0, 0, 0, 1, 0, 1, 0, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2,
            1, 2, 1, 0, 1, 0, 2, 0, 0, 2, 1, 2, 2, 2, 1, 2, 0, 2, 2, 2, 2, 2, 1, 2, 2, 1, 1, 0, 1,
            0, 1, 0, 0, 1, 1, 2, 2, 2, 1, 0, 1, 0, 0, 2, 0, 1, 0, 1, 1, 2, 0, 1, 2, 0, 0, 1, 0, 2,
            0, 1, 2, 2, 1, 1, 1, 1, 1, 0, 0, 1, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 2, 2, 0, 1, 0, 1, 1,
            2, 2, 1, 1, 1, 2, 1, 1, 2, 2, 2, 1, 2, 1, 0, 1, 2, 1, 2, 2, 0, 0, 2, 1, 0, 2, 1, 2, 2,
            0, 0, 2, 2, 2, 1, 0, 2, 1, 1, 1, 1, 2, 1, 0, 1, 0, 0, 1, 1, 2, 1, 1, 2, 1, 1, 0, 2, 2,
            0, 2, 1, 1, 0, 1, 0, 2, 0, 2, 2, 0, 1, 2, 1, 0,
        ]),
    );
    let c: Seq<i128> = Seq::from_native_slice(&[
        1857, 1746, 1704, -753, 1794, -1323, 1461, -2259, 2310, 1293, -753, 492, -1767, 912, 135,
        -708, 201, 351, 2283, -2241, 1380, -1800, -624, -1890, 132, -2190, -453, -723, 285, -816,
        -2214, 1404, 1215, -1896, 444, -189, -1782, -954, -861, 1668, 393, -1629, -723, 2259, -570,
        -381, 1527, 81, 795, 525, 2304, -1161, 603, -978, -1509, 1533, 18, 1743, 1929, -1176,
        -1980, -1680, 1557, -1161, -942, 678, -2208, -1956, -825, 1212, -330, -996, 594, 2220, 750,
        -1092, -1158, -480, 1464, 1044, -480, 507, -1086, 2127, -24, 180, -1383, 1842, -2010, 135,
        -1845, -1710, -756, 2250, 2268, -1674, 1398, -963, -924, -702, -435, 2211, 2121, -966, 717,
        -2100, 108, 1017, 2310, -57, 189, 1440, -2010, 2058, 831, 1236, 2061, -939, 513, -1653,
        681, -2049, -1854, 57, 225, 495, -1236, 2154, 1413, 1662, -1227, 666, -1365, -1788, -1914,
        1920, 510, -2211, 1545, 1647, 600, -2031, -456, 1185, -2292, -804, -1761, -384, -1206, 372,
        -1338, -2217, 1248, -801, 1455, 951, 1770, -108, 939, -888, 1104, 219, 1731, 1200, -2187,
        1479, -1284, 2088, 1947, 1635, 1296, -2094, 501, -1299, 51, 1503, -867, -1761, -1815, 987,
        -648, -1503, 627, 2067, -48, 1449, 1539, -1065, 780, -1125, -2145, 2265, -264, -792, -2202,
        903, -1320, -384, 945, -1779, 540, 42, 351, 1149, -1698, -1344, -183, 294, -1407, -417,
        126, -495, -1671, 564, 69, -168, -783, -1365, 846, -273, -831, -948, -1578, -15, -822,
        -1413, 2241, -1893, 2283, 1863, 642, 855, 471, 1794, 1062, -1791, 627, -15, -987, 591, 804,
        1641, -798, -573, -183, 1734, -2070, -2031, -1695, 2178, 2103, -12, 459, -1545, 1680, 741,
        33, -438, -606, 786, 1374, 1089, -2154, -1785, 972, 1908, 1938, -1026, -39, 483, -954,
        -849, 2001, -921, 201, -342, 708, -747, 123, -1335, -30, -423, -1452, -852, -2184, 2148,
        -1734, -591, -1110, 1563, 1686, -54, -1122, -141, -663, -933, 2034, -1713, -696, 2052,
        1452, -1308, -2283, -1737, 867, -117, 1026, -267, -570, -2049, 1827, 1737, 1644, 2250,
        -810, 1008, -630, 1065, 1425, 822, -942, -930, -1161, 1917, -699, -1341, -1710, -1380,
        1029, 1968, 1728, 675, 1920, -2178, 216, -453, -1401, 1599, 588, -2244, -885, 2211, 2196,
        -1557, -1650, 1863, 1044, -465, 183, 600, -2277, 1590, -459, 1104, -396, 1230, -663, -1302,
        1539, 684, 1167, 168, -1032, -702, -291, -621, 369, -1200, -1884, -1590, -36, 1713, -2202,
        114, 1602, 297, -1911, 1446, -132, -717, 867, 1536, -1686, -2151, 2214, -168, -1290, 147,
        1995, 1893, -1587, -579, 1767, -2193, 1611, 747, -1923, 1695, 1905, 444, 1944, -411, 1233,
        165, 1857, -1065, -1419, 405, 63, 1176, -354, 834, 1656, -1338, -1497, 1893, 1935, -1875,
        -318, 1920, 6, 168, 1407, 207, 1623, 1314, 1446, -1788, -600, 1608, 486, 1368, 1959, 564,
        1236, -612, -2052, -1464, -1116, 2235, 78, -1191, 168, 1410, 2145, 618, 294, 1275, 2052,
        -1062, 2160, -402, 2151, -429, -1539, 2082, -1125, -1104, 1602, -546, 72, 942, 1746, -1563,
        1455, -2295, -240, -186, 876, 1794, 420, -201, 1632, 123, 1776, -933, 939, 1602, 2079,
        2037, -1245, -315, -1905, 2157, 873, -183, 57, 2223, -2193, -1767, 30, -1452, 1323, 252,
        2283, -2301, -474, -1434, -12, -909, -1728, 2127, -1194, 912, 315, -1749, 1170, 366, 1635,
        -183, 2301, 1626, 1653, 621, 492, 1590, -195, -666, -267, 1929, -1533, 2007, 1929, -114,
        2244, 1311, -1851, -123, -1221, -2052, 1956, -1689, 978, -1776, 1629, -873, -2100, 993,
        309, 1851, 912, -1116, -2133, 18, -1227, 84, -1509, -438, -132, -501, 615, 282, 1860, 1680,
        1122, 2244, -375, -2073, 1353, -1704, 1293, 1950, 324, -1410, 132, -261, -1896, 1440, -342,
        -1848, 1698, 273, 2265, -249, 168, 1395, -1050, 117, -2163, -12, -2142, 1725, 1176, 1998,
        -2256, 1623, -1383, 1287, -555, -1968, -888, 1695, -933, 105, 429, 2052, -18, 435, -660,
        1194, -42, 2256, 27, 963, 1782, -1593, 900, 75, 597, 534, -1815, 600, 315, -1113, -1533,
        -159, 1302, -531, 2130, 489, 1140, -1179, -330, 1911, 780, -1260, -1530, 165, 477, 1755,
        -2118, -1380, 636, -1893, -1683, -1836, 1965, -1752, 738, -753, 1512, -786, 621, -2244,
        -2076, 1008, 753, -1458, -672, -1782, -1695, -780, -1596, 0,
    ]);

    let cipher = poly_encrypt(&m, &pk);
    assert_eq!(cipher.len(), c.len());
    for i in 0..cipher.len() {
        assert_eq!(cipher[i], c[i]);
    }

    let (message, ok) = poly_decrypt(&cipher, &sk.0, &sk.1);
    assert_eq!(ok, true);
    assert_eq!(message.len(), m.len());
    for i in 0..message.len() {
        assert_eq!(message[i], m[i]);
    }
}
