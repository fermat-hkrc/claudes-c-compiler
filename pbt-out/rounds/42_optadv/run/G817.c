static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 275u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[3 % 6] = 829; cells[6] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 623u; }
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 410u; }
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 96u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }