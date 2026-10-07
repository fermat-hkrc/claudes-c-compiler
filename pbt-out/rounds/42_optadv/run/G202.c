static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 552u; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 483u; }
    arr[2 % 6] = 199; cells[2] += arr[2 % 6];
    arr[7 % 6] = 373; cells[5] += arr[7 % 6];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 716u; }
    arr[3 % 6] = 643; cells[4] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }