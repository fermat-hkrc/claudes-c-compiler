# INDEX — x86 intrinsics missing (immintrin.h layer), 6 batches

Witness per issue: `gcc -m<flag>` builds+runs the snippet; ccc implicit-decl -> undefined symbol. Verified-working contrast: SSE1/2 float base 23/23, SHA family, RDRAND/RDTSC/__cpuid.

| # | ledger name |
|---|---|
| [#1793](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1793) | SSE3 intrinsics missing (_mm_addsub_ps) |
| [#1794](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1794) | SSE4.1 blend intrinsics missing (_mm_blend_epi16) |
| [#1795](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1795) | SSE4.1 dot-product intrinsics missing (_mm_dp_ps) |
| [#1796](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1796) | POPCNT intrinsics missing (_mm_popcnt_u64) |
| [#1797](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1797) | AVX intrinsics missing (_mm256_add_ps / _mm256_cvtss_f32) |
| [#1798](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1798) | AVX2 gather intrinsics missing (_mm_i32gather_epi32) |
| [#1799](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1799) | BMI tzcnt intrinsics missing (_tzcnt_u32) |
| [#1800](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1800) | LZCNT intrinsics missing (_lzcnt_u32) |
| [#1801](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1801) | F16C intrinsics missing (_mm_cvtps_ph) |
| [#1802](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1802) | AVX-512F intrinsics missing (_mm512_add_epi32 / _mm512_mask_add_epi32 / _mm512_castsi512_si128) |
| [#1803](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1803) | XOP intrinsics missing (_mm_roti_epi32) |
| [#1804](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1804) | FMA4 intrinsics missing (_mm_macc_ps) |
| [#1805](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1805) | intrinsic _mm_movedup_pd missing |
| [#1806](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1806) | intrinsic _mm_minpos_epu16 missing |
| [#1807](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1807) | intrinsic _mm_ceil_pd / _mm_ceil_ps missing |
| [#1808](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1808) | intrinsic _mm_round_ps / _mm_round_pd missing |
| [#1809](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1809) | intrinsic _mm256_sub_pd missing |
| [#1810](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1810) | intrinsic _mm256_mul_ps missing |
| [#1811](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1811) | intrinsic _mm256_zeroupper missing |
| [#1812](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1812) | intrinsic _mm256_broadcastb_epi8 missing |
| [#1813](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1813) | intrinsic _mm256_avg_epu8 missing |
| [#1814](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1814) | intrinsic _blsr_u32 missing |
| [#1815](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1815) | intrinsic _bextr_u32 missing |
| [#1816](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1816) | intrinsic _tzcnt_u64 missing |
| [#1817](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1817) | intrinsic _mm_lddqu_si128 missing |
| [#1818](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1818) | intrinsic _mm256_cmp_ps + _mm256_movemask_ps missing |
| [#1819](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1819) | intrinsic _mm256_blend_ps missing |
| [#1820](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1820) | intrinsic _mm_dp_pd missing |
| [#1821](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1821) | intrinsic _mm256_permute2f128_pd missing |
| [#1822](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1822) | intrinsic _mm256_testz_si256 missing |
| [#1823](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1823) | intrinsic _mm_fmsub_ps missing |
| [#1824](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1824) | intrinsic _mm_fmaddsub_ps / _mm_fmsubadd_ps missing |
| [#1825](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1825) | intrinsic _mm_fnmadd_ps / _mm_fnmsub_ps missing |
| [#1826](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1826) | intrinsic _blsi_u64 missing |
| [#1827](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1827) | intrinsic _blsmsk_u64 missing |
| [#1828](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1828) | intrinsic _bextr_u64 missing |
| [#1829](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1829) | intrinsic _mulx_u64 missing |
| [#1830](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1830) | intrinsic _mm256_sad_epu8 missing |
| [#1831](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1831) | intrinsic _mm256_sign_epi16 missing |
| [#1832](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1832) | intrinsic _mm256_packus_epi32 missing |
| [#1833](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1833) | intrinsic _andn_u64 missing |
| [#1834](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1834) | intrinsic _mm_move_epi64 missing |
| [#1835](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1835) | intrinsic _mm_mul_epi32 missing |
| [#1836](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1836) | intrinsic _mm256_sqrt_ps missing |
| [#1837](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1837) | intrinsic _mm256_max_ps / _mm256_min_ps missing |
| [#1838](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1838) | intrinsic _mm256_and_ps (and _or_ps/_xor_ps) missing |
| [#1839](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1839) | intrinsic _mm256_sllv_epi32 missing |
| [#1840](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1840) | intrinsic _mm256_srlv_epi32 missing |
| [#1841](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1841) | intrinsic _mm256_srav_epi32 missing |
| [#1842](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1842) | intrinsic _mm256_movemask_ps missing |
| [#1843](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1843) | intrinsic _mm256_cvtepi8_epi16 (and the 7 other sign-extend family members) missing |
| [#1844](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1844) | intrinsic _mm_fmadd_ss (and the _ss scalar family) missing |
| [#1845](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1845) | intrinsic _mm_fmadd_sd (and the _sd scalar family) missing |
| [#1846](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1846) | intrinsic __readeflags / __writeeflags missing |
| [#1847](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1847) | intrinsic _mm512_sub_epi32 missing |
| [#1848](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1848) | intrinsic _mm512_mullo_epi32 missing |
| [#1849](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1849) | intrinsic _mm512_mul_epu32 missing |
| [#1850](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1850) | intrinsic _mm512_ternarylogic_epi32 missing |
| [#1851](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1851) | intrinsic _mm512_maskz_add_epi32 missing |
| [#1852](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1852) | intrinsic _mm512_min_epi32 / _mm512_max_epi32 missing |
| [#1853](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1853) | intrinsic _mm512_srli_epi32 (and the shift family) missing |
| [#1854](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1854) | intrinsic _mm512_cmpgt_epi32_mask missing |
| [#1855](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1855) | intrinsic _mm512_add_ps missing |
| [#1856](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1856) | intrinsic _mm512_mul_ps missing |
| [#1857](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1857) | intrinsic _mm512_fmadd_ps missing |
| [#1858](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1858) | intrinsic _mm512_cvtepi32_ps missing |
| [#1859](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1859) | intrinsic _kand_mask16 / _kortestz_mask16_u8 (k-op intrinsics) missing |
| [#1860](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1860) | intrinsic _mm512_add_epi8 missing |
| [#1861](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1861) | intrinsic _mm512_add_epi16 missing |
| [#1862](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1862) | intrinsic _mm512_madd_epi16 missing |
| [#1863](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1863) | intrinsic _mm512_cvtepi8_epi16 (512-bit sign-extend family) missing |
| [#1864](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1864) | intrinsic _mm256_mask_add_epi32 missing |
| [#1865](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1865) | intrinsic _mm_gf2p8mul_epi8 missing |
| [#1866](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1866) | intrinsic _mm_gf2p8affine_epi64_epi8 missing |
| [#1867](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1867) | intrinsic _mm512_andnot_si512 missing |
| [#1868](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1868) | intrinsic _mm512_madd52hi_epu64 missing |
| [#1869](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1869) | intrinsic _mm512_madd52lo_epu64 missing |
| [#1870](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1870) | intrinsic _mm512_permutexvar_epi8 missing |
| [#1871](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1871) | intrinsic _mm512_popcnt_epi32 / _mm512_popcnt_epi64 missing |
| [#1872](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1872) | intrinsic _mm512_dpbusd_epi32 / _mm512_dpbssd_epi32 missing |
| [#1873](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1873) | intrinsic _rdpid_u32 missing |
| [#1874](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1874) | intrinsic _mm_clwb / _mm_clflushopt intrinsic form missing |
