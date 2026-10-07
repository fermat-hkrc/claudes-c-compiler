static int f1(int *p, int *q) { *p += 731; return *q * 2; }
static int f2(int *p, int *q) { *p += 950; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[4]);
    arr[6 % 6] = 626; cells[1] += arr[6 % 6];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 962u; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 397u; }
    arr[2 % 6] = 722; cells[4] += arr[2 % 6];
    sum += f2(&cells[6], &cells[5]);
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 101u; }
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 415u; }
    cells[6] = cells[4] + 619;
    cells[5] = cells[4] + 202;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 465u; }
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 553u; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[3] = cells[6] + 995;
    cells[6] = cells[3] + 228;
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 645u; }
    arr[3 % 6] = 373; cells[4] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }