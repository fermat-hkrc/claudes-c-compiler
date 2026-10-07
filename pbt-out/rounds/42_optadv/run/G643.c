static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    arr[2 % 6] = 830; cells[5] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[4] = cells[6] + 614;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[3] + 320;
    arr[5 % 6] = 177; cells[3] += arr[5 % 6];
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    cells[4] = cells[5] + 267;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 377u; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 359u; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 491u; }
    arr[3 % 6] = 728; cells[2] += arr[3 % 6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }