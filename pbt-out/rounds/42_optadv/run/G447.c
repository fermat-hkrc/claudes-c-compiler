static int f1(int *p, int *q) { *p += 444; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 27; cells[4] += arr[7 % 6];
    cells[4] = cells[7] + 549;
    cells[5] = cells[4] + 345;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 337u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 57u; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 51u; }
    sum += f1(&cells[6], &cells[7]);
    arr[5 % 6] = 472; cells[2] += arr[5 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }