static int f1(int *p, int *q) { *p += 523; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 582; cells[7] += arr[0 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    cells[1] = cells[2] + 539;
    sum += f1(&cells[4], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }