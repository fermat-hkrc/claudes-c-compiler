static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 596; cells[4] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[4] = cells[3] + 10;
    arr[4 % 6] = 844; cells[0] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    cells[5] = cells[3] + 137;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[4 % 6] = 718; cells[1] += arr[4 % 6];
    arr[6 % 6] = 928; cells[1] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 129u; }
    arr[5 % 6] = 574; cells[1] += arr[5 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[4] = cells[2] + 157;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 214u; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }