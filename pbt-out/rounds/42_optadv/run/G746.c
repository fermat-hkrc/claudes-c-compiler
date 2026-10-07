static int f1(int *p, int *q) { *p += 993; return *q * 2; }
static int f2(int *p, int *q) { *p += 628; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[6]);
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    arr[0 % 6] = 219; cells[7] += arr[0 % 6];
    arr[5 % 6] = 721; cells[4] += arr[5 % 6];
    cells[7] = cells[0] + 656;
    cells[3] = cells[1] + 662;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 53u; }
    cells[2] = cells[0] + 296;
    sum += f2(&cells[7], &cells[1]);
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    arr[0 % 6] = 788; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }