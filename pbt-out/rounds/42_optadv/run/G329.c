static int f1(int *p, int *q) { *p += 933; return *q * 2; }
static int f2(int *p, int *q) { *p += 26; return *q * 2; }
static int f3(int *p, int *q) { *p += 14; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 528u; }
    sum += f1(&cells[0], &cells[6]);
    sum += f2(&cells[4], &cells[2]);
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 182u; }
    arr[7 % 6] = 69; cells[3] += arr[7 % 6];
    arr[6 % 6] = 137; cells[3] += arr[6 % 6];
    sum += f3(&cells[5], &cells[7]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }