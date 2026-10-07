static int f1(int *p, int *q) { *p += 809; return *q * 2; }
static int f2(int *p, int *q) { *p += 882; return *q * 2; }
static int f3(int *p, int *q) { *p += 883; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 44; cells[2] += arr[6 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 245u; }
    sum += f1(&cells[3], &cells[2]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 917u; }
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += f2(&cells[6], &cells[1]);
    sum += f3(&cells[1], &cells[6]);
    arr[4 % 6] = 212; cells[1] += arr[4 % 6];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 77u; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }