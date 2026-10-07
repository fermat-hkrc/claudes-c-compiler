static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[1] + 925;
    arr[2 % 6] = 424; cells[3] += arr[2 % 6];
    arr[4 % 6] = 597; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[6] = cells[6] + 957;
    arr[2 % 6] = 926; cells[2] += arr[2 % 6];
    arr[4 % 6] = 627; cells[7] += arr[4 % 6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 776u; }
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 48u; }
    cells[4] = cells[4] + 680;
    arr[2 % 6] = 484; cells[6] += arr[2 % 6];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }