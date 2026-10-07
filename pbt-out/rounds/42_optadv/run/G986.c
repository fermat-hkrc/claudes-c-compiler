static int f1(int *p, int *q) { *p += 65; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[5] = cells[5] + 843;
    arr[5 % 6] = 334; cells[5] += arr[5 % 6];
    cells[7] = cells[7] + 242;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 558u; }
    cells[5] = cells[5] + 46;
    arr[5 % 6] = 989; cells[5] += arr[5 % 6];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 368u; }
    arr[5 % 6] = 162; cells[7] += arr[5 % 6];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += f1(&cells[5], &cells[6]);
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    arr[1 % 6] = 892; cells[1] += arr[1 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }