static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[5] = cells[7] + 481;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    cells[2] = cells[1] + 655;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[1] = cells[1] + 48;
    cells[2] = cells[1] + 111;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[7 % 6] = 346; cells[4] += arr[7 % 6];
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 893u; }
    cells[0] = cells[1] + 38;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    arr[3 % 6] = 913; cells[2] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }