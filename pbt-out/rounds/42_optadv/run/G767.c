static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 733; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 201u; }
    static int f1(int *p, int *q) { *p += 838; return *q * 2; }
    sum += f1(&cells[0], &cells[7]);
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[6 % 6] = 529; cells[1] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }