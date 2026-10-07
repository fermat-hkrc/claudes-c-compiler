static int f1(int *p, int *q) { *p += 328; return *q * 2; }
static int f2(int *p, int *q) { *p += 291; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 913; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 898u; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += f1(&cells[0], &cells[5]);
    arr[3 % 6] = 465; cells[6] += arr[3 % 6];
    sum += f2(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    cells[5] = cells[4] + 399;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 717u; }
    arr[1 % 6] = 122; cells[4] += arr[1 % 6];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    arr[7 % 6] = 10; cells[2] += arr[7 % 6];
    arr[6 % 6] = 262; cells[1] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }