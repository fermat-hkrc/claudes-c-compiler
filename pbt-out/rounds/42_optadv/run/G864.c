static int f1(int *p, int *q) { *p += 595; return *q * 2; }
static int f2(int *p, int *q) { *p += 819; return *q * 2; }
static int f3(int *p, int *q) { *p += 594; return *q * 2; }
static int f4(int *p, int *q) { *p += 11; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[2] + 830;
    sum += f1(&cells[2], &cells[3]);
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 648u; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    arr[1 % 6] = 690; cells[2] += arr[1 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 558u; }
    arr[2 % 6] = 173; cells[7] += arr[2 % 6];
    sum += f2(&cells[0], &cells[3]);
    sum += f3(&cells[1], &cells[2]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 619u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 689u; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 795u; }
    sum += f4(&cells[7], &cells[7]);
    arr[6 % 6] = 409; cells[3] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }