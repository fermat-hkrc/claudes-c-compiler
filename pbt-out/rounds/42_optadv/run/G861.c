static int f1(int *p, int *q) { *p += 933; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    cells[6] = cells[1] + 168;
    sum += f1(&cells[4], &cells[5]);
    arr[2 % 6] = 83; cells[4] += arr[2 % 6];
    cells[0] = cells[6] + 953;
    arr[7 % 6] = 73; cells[1] += arr[7 % 6];
    arr[4 % 6] = 501; cells[0] += arr[4 % 6];
    arr[3 % 6] = 72; cells[0] += arr[3 % 6];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 660u; }
    cells[7] = cells[4] + 958;
    cells[0] = cells[3] + 157;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 829u; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 902u; }
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    arr[6 % 6] = 911; cells[2] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }