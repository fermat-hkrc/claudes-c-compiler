static int f1(int *p, int *q) { *p += 243; return *q * 2; }
static int f2(int *p, int *q) { *p += 472; return *q * 2; }
static int f3(int *p, int *q) { *p += 461; return *q * 2; }
static int f4(int *p, int *q) { *p += 939; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[0] = cells[3] + 558;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 849u; }
    cells[2] = cells[0] + 389;
    arr[5 % 6] = 135; cells[5] += arr[5 % 6];
    arr[6 % 6] = 727; cells[1] += arr[6 % 6];
    arr[0 % 6] = 185; cells[0] += arr[0 % 6];
    sum += f1(&cells[1], &cells[7]);
    sum += f2(&cells[7], &cells[2]);
    sum += f3(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += f4(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[7 % 6] = 80; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 60u; }
    cells[0] = cells[4] + 45;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }