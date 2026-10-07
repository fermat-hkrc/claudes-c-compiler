static int f1(int *p, int *q) { *p += 933; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[6 % 6] = 872; cells[4] += arr[6 % 6];
    cells[4] = cells[3] + 411;
    cells[2] = cells[7] + 779;
    cells[6] = cells[3] + 29;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 814u; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[5], &cells[6]);
    arr[3 % 6] = 944; cells[0] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 696u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }