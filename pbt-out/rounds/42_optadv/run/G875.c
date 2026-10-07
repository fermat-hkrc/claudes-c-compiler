static int f1(int *p, int *q) { *p += 751; return *q * 2; }
static int f2(int *p, int *q) { *p += 693; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[3 % 6] = 123; cells[7] += arr[3 % 6];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 952u; }
    arr[1 % 6] = 622; cells[6] += arr[1 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[0 % 6] = 266; cells[1] += arr[0 % 6];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 855u; }
    arr[3 % 6] = 641; cells[6] += arr[3 % 6];
    arr[3 % 6] = 329; cells[6] += arr[3 % 6];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += f1(&cells[0], &cells[1]);
    arr[5 % 6] = 807; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[7 % 6] = 246; cells[1] += arr[7 % 6];
    sum += f2(&cells[0], &cells[6]);
    arr[7 % 6] = 591; cells[0] += arr[7 % 6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 537u; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }