static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    cells[1] = cells[3] + 819;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[0] + 224;
    arr[0 % 6] = 393; cells[0] += arr[0 % 6];
    arr[4 % 6] = 208; cells[3] += arr[4 % 6];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 621u; }
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[7 % 6] = 387; cells[6] += arr[7 % 6];
    arr[3 % 6] = 355; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }