static int f1(int *p, int *q) { *p += 262; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[5] + 793;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    cells[2] = cells[7] + 283;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    arr[2 % 6] = 692; cells[1] += arr[2 % 6];
    cells[3] = cells[5] + 218;
    sum += f1(&cells[2], &cells[6]);
    arr[6 % 6] = 576; cells[3] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }