static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    static int f1(int *p, int *q) { *p += 110; return *q * 2; }
    sum += f1(&cells[5], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    cells[6] = cells[4] + 28;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[0] = cells[0] + 937;
    arr[0 % 6] = 843; cells[3] += arr[0 % 6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 419u; }
    cells[4] = cells[2] + 767;
    cells[6] = cells[7] + 391;
    arr[6 % 6] = 594; cells[0] += arr[6 % 6];
    arr[1 % 6] = 427; cells[4] += arr[1 % 6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[4] = cells[7] + 284;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }