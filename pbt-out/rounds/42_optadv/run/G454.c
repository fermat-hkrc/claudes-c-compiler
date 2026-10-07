static int f1(int *p, int *q) { *p += 766; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 58; cells[5] += arr[7 % 6];
    arr[6 % 6] = 132; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[0 % 6] = 993; cells[6] += arr[0 % 6];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[4 % 6] = 966; cells[1] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += f1(&cells[4], &cells[0]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 365u; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 968u; }
    cells[7] = cells[0] + 177;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }