static int f1(int *p, int *q) { *p += 228; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 147u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 342u; }
    cells[6] = cells[2] + 466;
    cells[1] = cells[1] + 160;
    sum += f1(&cells[3], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }