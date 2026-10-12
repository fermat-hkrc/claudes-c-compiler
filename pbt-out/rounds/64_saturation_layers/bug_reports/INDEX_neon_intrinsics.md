# INDEX — NEON intrinsics missing (include/arm_neon.h layer), 6 batches

Witness per issue: clang-21 --target=aarch64 compiles; ccc-arm implicit-decl -> undefined symbol; `grep -w <name> include/arm_neon.h` no match. Layer note: 12/12 sampled encoders of these work — header-layer gap, not encode_neon_*.

| # | ledger name |
|---|---|
| [#1875](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1875) | NEON intrinsic vmulq_f32 missing |
| [#1876](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1876) | NEON intrinsic vmulq_u8 missing |
| [#1877](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1877) | NEON intrinsic vld1q_u32 missing |
| [#1878](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1878) | NEON intrinsic vst1q_u32 missing |
| [#1879](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1879) | NEON intrinsic vorrq_s32 missing |
| [#1880](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1880) | NEON intrinsic vmaxq_s32 missing |
| [#1881](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1881) | NEON intrinsic vminq_f32 missing |
| [#1882](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1882) | NEON intrinsic vabsq_s16 missing |
| [#1883](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1883) | NEON intrinsic vnegq_s32 missing |
| [#1884](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1884) | NEON intrinsic vmlaq_f32 missing |
| [#1885](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1885) | NEON intrinsic vfmaq_f32 missing |
| [#1886](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1886) | NEON intrinsic vpaddq_u32 missing |
| [#1887](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1887) | NEON intrinsic vaddvq_u32 missing |
| [#1888](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1888) | NEON intrinsic vaddvq_s8 missing |
| [#1889](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1889) | NEON intrinsic vmaxvq_f32 missing |
| [#1890](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1890) | NEON intrinsic vcgtq_s32 missing |
| [#1891](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1891) | NEON intrinsic vreinterpretq_f32_u32 missing |
| [#1892](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1892) | NEON intrinsic vcvtq_f32_s32 missing |
| [#1893](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1893) | NEON intrinsic vrndq_f32 missing |
| [#1894](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1894) | NEON intrinsic vsqrtq_f32 missing |
| [#1895](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1895) | NEON intrinsic vqaddq_s8 missing |
| [#1896](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1896) | NEON intrinsic vqdmulhq_s16 missing |
| [#1897](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1897) | NEON intrinsic vcvtq_u32_f32 missing |
| [#1898](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1898) | NEON intrinsic vrecpeq_f32 missing |
| [#1899](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1899) | NEON intrinsic vshrq_n_s64 missing |
| [#1900](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1900) | NEON intrinsic vld2q_u32 missing (b2) |
| [#1901](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1901) | NEON intrinsic vld3_s8 missing (b2) |
| [#1902](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1902) | NEON intrinsic vst2_u16 missing (b2) |
| [#1903](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1903) | NEON intrinsic vuzp1q_s16 missing (b2) |
| [#1904](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1904) | NEON intrinsic vtbl1_s8 missing (b2) |
| [#1905](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1905) | NEON intrinsic vtbx1_s8 missing (b2) |
| [#1906](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1906) | NEON intrinsic vaddl_s16 missing (b2) |
| [#1907](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1907) | NEON intrinsic vaddw_s16 missing (b2) |
| [#1908](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1908) | NEON intrinsic vsubl_u16 missing (b2) |
| [#1909](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1909) | NEON intrinsic vmull_s16 missing (b2) |
| [#1910](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1910) | NEON intrinsic vmull_u8 missing (b2) |
| [#1911](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1911) | NEON intrinsic vshll_n_s16 missing (b2) |
| [#1912](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1912) | NEON intrinsic vrecpsq_f32 missing (b2) |
| [#1913](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1913) | NEON intrinsic vclzq_s32 missing (b2) |
| [#1914](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1914) | NEON intrinsic vclsq_s8 missing (b2) |
| [#1915](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1915) | NEON intrinsic vpadalq_s16 missing (b2) |
| [#1916](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1916) | NEON intrinsic vcvtq_n_s32_f32 missing (b2) |
| [#1917](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1917) | NEON intrinsic vcvtq_n_f32_s32 missing (b2) |
| [#1918](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1918) | NEON intrinsic vshlq_s64 missing (b2) |
| [#1919](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1919) | NEON intrinsic vqshlq_s32 missing (b2) |
| [#1920](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1920) | NEON intrinsic vpmaxq_f32 missing (b2) |
| [#1921](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1921) | NEON intrinsic vcreate_s8 missing (b2) |
| [#1922](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1922) | NEON intrinsic vld1q_f64 missing (b2) |
| [#1923](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1923) | NEON intrinsic vcageq_f32 missing (b2) |
| [#1924](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1924) | NEON intrinsic vaddq_f64 missing (b3) |
| [#1925](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1925) | NEON intrinsic vsubq_f64 missing (b3) |
| [#1926](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1926) | NEON intrinsic vmulq_f64 missing (b3) |
| [#1927](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1927) | NEON intrinsic vmaxq_f64 missing (b3) |
| [#1928](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1928) | NEON intrinsic vsqrtq_f64 missing (b3) |
| [#1929](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1929) | NEON intrinsic vbicq_u32 missing (b3) |
| [#1930](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1930) | NEON intrinsic vmvnq_u32 missing (b3) |
| [#1931](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1931) | NEON intrinsic vornq_u32 missing (b3) |
| [#1932](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1932) | NEON intrinsic vrev64q_u32 missing (b3) |
| [#1933](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1933) | NEON intrinsic vqsubq_s8 missing (b3) |
| [#1934](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1934) | NEON intrinsic vrshr_n_s32 missing (b3) |
| [#1935](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1935) | NEON intrinsic vshlq_n_s64 missing (b3) |
| [#1936](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1936) | NEON intrinsic vaddlvq_u32 missing (b3) |
| [#1937](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1937) | NEON intrinsic vld1q_lane_u64 missing (b3) |
| [#1938](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1938) | NEON intrinsic vqmovun_s32 missing (b3) |
| [#1939](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1939) | NEON intrinsic vaddhn_s32 missing (b3) |
| [#1940](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1940) | NEON intrinsic vrshrn_n_s32 missing (b3) |
| [#1941](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1941) | NEON intrinsic vsraq_n_s32 missing (b3) |
| [#1942](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1942) | NEON intrinsic vaddq_f16 missing (b3) |
| [#1943](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1943) | NEON intrinsic vmulq_f16 missing (b3) |
| [#1944](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1944) | NEON intrinsic vcvt_f64_f32 missing (b3) |
| [#1945](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1945) | NEON intrinsic vcvt_f32_f64 missing (b3) |
| [#1946](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1946) | NEON intrinsic vld1_u32 missing (b4) |
| [#1947](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1947) | NEON intrinsic vld1_s8 missing (b4) |
| [#1948](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1948) | NEON intrinsic vst1_u32 missing (b4) |
| [#1949](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1949) | NEON intrinsic vst1q_lane_u32 missing (b4) |
| [#1950](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1950) | NEON intrinsic vsubhn_s32 missing (b4) |
| [#1951](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1951) | NEON intrinsic vabdq_s32 missing (b4) |
| [#1952](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1952) | NEON intrinsic vabdq_f32 missing (b4) |
| [#1953](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1953) | NEON intrinsic vceqzq_s32 missing (b4) |
| [#1954](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1954) | NEON intrinsic vcgtzq_f32 missing (b4) |
| [#1955](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1955) | NEON intrinsic vcaleq_f32 missing (b4) |
| [#1956](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1956) | NEON intrinsic vpadd_f32 missing (b4) |
| [#1957](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1957) | NEON intrinsic vpmax_f32 missing (b4) |
| [#1958](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1958) | NEON intrinsic vpmin_s32 missing (b4) |
| [#1959](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1959) | NEON intrinsic vqshlq_n_s32 missing (b4) |
| [#1960](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1960) | NEON intrinsic vqshrun_n_s32 missing (b4) |
| [#1961](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1961) | NEON intrinsic vqrshrn_n_s32 missing (b4) |
| [#1962](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1962) | NEON intrinsic vshr_n_s8 missing (b4) |
| [#1963](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1963) | NEON intrinsic vshl_s8 missing (b4) |
| [#1964](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1964) | NEON intrinsic vnegq_f64 missing (b4) |
| [#1965](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1965) | NEON intrinsic vabsq_f64 missing (b4) |
| [#1966](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1966) | NEON intrinsic vrecpe_f32 missing (b4) |
| [#1967](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1967) | NEON intrinsic vrsqrte_f32 missing (b4) |
| [#1968](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1968) | NEON intrinsic vrsqrtsq_f32 missing (b4) |
| [#1969](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1969) | NEON intrinsic vdupq_n_s64 missing (b4) |
| [#1970](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1970) | NEON intrinsic vdup_n_f64 missing (b4) |
| [#1971](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1971) | NEON intrinsic vsetq_lane_f64 missing (b4) |
| [#1972](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1972) | NEON intrinsic vcopyq_laneq_s32 missing (b4) |
| [#1973](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1973) | NEON intrinsic vcvtmq_s32_f32 missing (b4) |
| [#1974](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1974) | NEON intrinsic vrndmq_f32 missing (b4) |
| [#1975](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1975) | NEON intrinsic vrndpq_f32 missing (b4) |
| [#1976](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1976) | NEON intrinsic vrndaq_f32 missing (b4) |
| [#1977](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1977) | NEON intrinsic vmaxnmq_f32 missing (b4) |
| [#1978](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1978) | NEON intrinsic vminnmq_f64 missing (b4) |
| [#1979](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1979) | NEON intrinsic vreinterpretq_u32_s32 missing (b4) |
| [#1980](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1980) | NEON intrinsic vaddl_high_s16 missing (b5) |
| [#1981](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1981) | NEON intrinsic vmull_high_s16 missing (b5) |
| [#1982](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1982) | NEON intrinsic vaddw_high_s16 missing (b5) |
| [#1983](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1983) | NEON intrinsic vsubl_high_u16 missing (b5) |
| [#1984](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1984) | NEON intrinsic vshll_high_n_s16 missing (b5) |
| [#1985](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1985) | NEON intrinsic vmlal_s16 missing (b5) |
| [#1986](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1986) | NEON intrinsic vmlsl_s16 missing (b5) |
| [#1987](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1987) | NEON intrinsic vmlsq_f32 missing (b5) |
| [#1988](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1988) | NEON intrinsic vfmaq_lane_f32 missing (b5) |
| [#1989](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1989) | NEON intrinsic vmlaq_lane_f32 missing (b5) |
| [#1990](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1990) | NEON intrinsic vld1_dup_u32 missing (b5) |
| [#1991](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1991) | NEON intrinsic vqabsq_s8 missing (b5) |
| [#1992](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1992) | NEON intrinsic vqnegq_s8 missing (b5) |
| [#1993](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1993) | NEON intrinsic vrshlq_s32 missing (b5) |
| [#1994](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1994) | NEON intrinsic vrshrq_n_s32 missing (b5) |
| [#1995](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1995) | NEON intrinsic vshrn_n_s16 missing (b5) |
| [#1996](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1996) | NEON intrinsic vext_u16 missing (b5) |
| [#1997](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1997) | NEON intrinsic vzip1_u32 missing (b5) |
| [#1998](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1998) | NEON intrinsic vuzp1_u32 missing (b5) |
| [#1999](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1999) | NEON intrinsic vtrn1_s16 missing (b5) |
| [#2000](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2000) | NEON intrinsic vrev64q_u8 missing (b5) |
| [#2001](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2001) | NEON intrinsic vabal_s16 missing (b5) |
| [#2002](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2002) | NEON intrinsic vcagtq_f32 missing (b5) |
| [#2003](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2003) | NEON intrinsic vcaltq_f32 missing (b5) |
| [#2004](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2004) | NEON intrinsic vmulq_s8 missing (b6) |
| [#2005](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2005) | NEON intrinsic vmulq_s32 missing (b6) |
| [#2006](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2006) | NEON intrinsic vaddq_s64 missing (b6) |
| [#2007](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2007) | NEON intrinsic vsubq_s64 missing (b6) |
| [#2008](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2008) | NEON intrinsic vandq_s64 missing (b6) |
| [#2009](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2009) | NEON intrinsic vmulq_n_f32 missing (b6) |
| [#2010](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2010) | NEON intrinsic vmlaq_n_s32 missing (b6) |
| [#2011](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2011) | NEON intrinsic vqdmulhq_lane_s16 missing (b6) |
| [#2012](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2012) | NEON intrinsic vqrdmulhq_s16 missing (b6) |
| [#2013](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2013) | NEON intrinsic vqdmlal_s16 missing (b6) |
| [#2014](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2014) | NEON intrinsic vqdmlsl_s16 missing (b6) |
| [#2015](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2015) | NEON intrinsic vdupq_lane_u32 missing (b6) |
| [#2016](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2016) | NEON intrinsic vtst_s32 missing (b6) |
| [#2017](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2017) | NEON intrinsic vceq_s8 missing (b6) |
| [#2018](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2018) | NEON intrinsic vcgeq_f32 missing (b6) |
| [#2019](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2019) | NEON intrinsic vcleq_f32 missing (b6) |
| [#2020](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2020) | NEON intrinsic vcltq_f32 missing (b6) |
| [#2021](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2021) | NEON intrinsic vneg_f64 missing (b6) |
| [#2022](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2022) | NEON intrinsic vabs_f64 missing (b6) |
| [#2023](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2023) | NEON intrinsic vshlq_u8 missing (b6) |
| [#2024](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2024) | NEON intrinsic vabal_u16 missing (b6) |
| [#2025](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2025) | NEON intrinsic vsubw_s16 missing (b6) |
| [#2026](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2026) | NEON intrinsic vshll_n_u16 missing (b6) |
| [#2027](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2027) | NEON intrinsic vrsra_n_s32 missing (b6) |
| [#2028](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2028) | NEON intrinsic vpaddl_u8 missing (b6) |
| [#2029](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2029) | NEON intrinsic vpadal_s16 missing (b6) |
