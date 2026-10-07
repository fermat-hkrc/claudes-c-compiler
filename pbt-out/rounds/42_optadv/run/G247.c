static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 4u; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 483u; }
    arr[6 % 6] = 297; cells[5] += arr[6 % 6];
    cells[1] = cells[0] + 963;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 864u; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[5 % 6] = 596; cells[1] += arr[5 % 6];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 157u; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[7 % 6] = 851; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[1] = cells[2] + 804;
    cells[1] = cells[0] + 618;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }