static int f1(int *p, int *q) { *p += 494; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    cells[6] = cells[4] + 955;
    arr[2 % 6] = 380; cells[0] += arr[2 % 6];
    cells[7] = cells[1] + 70;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 504u; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += f1(&cells[2], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }