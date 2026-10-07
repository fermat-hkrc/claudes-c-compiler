static int f1(int *p, int *q) { *p += 536; return *q * 2; }
static int f2(int *p, int *q) { *p += 864; return *q * 2; }
static int f3(int *p, int *q) { *p += 661; return *q * 2; }
static int f4(int *p, int *q) { *p += 451; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 214; cells[1] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[7], &cells[4]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 844u; }
    sum += f2(&cells[4], &cells[7]);
    cells[0] = cells[6] + 83;
    sum += f3(&cells[5], &cells[2]);
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    sum += f4(&cells[7], &cells[1]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 752u; }
    cells[2] = cells[6] + 815;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }