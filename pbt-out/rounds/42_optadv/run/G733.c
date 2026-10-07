static int f1(int *p, int *q) { *p += 45; return *q * 2; }
static int f2(int *p, int *q) { *p += 218; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 966u; }
    sum += f1(&cells[0], &cells[1]);
    arr[0 % 6] = 949; cells[5] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[4 % 6] = 83; cells[3] += arr[4 % 6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[1 % 6] = 557; cells[5] += arr[1 % 6];
    arr[4 % 6] = 572; cells[6] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 45; cells[6] += arr[2 % 6];
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f2(&cells[1], &cells[7]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 682u; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }