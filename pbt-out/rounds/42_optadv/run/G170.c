static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[7] + 374;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    arr[2 % 6] = 706; cells[1] += arr[2 % 6];
    arr[1 % 6] = 128; cells[0] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }