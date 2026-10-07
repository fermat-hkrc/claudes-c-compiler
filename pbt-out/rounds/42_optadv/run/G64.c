static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[7] = cells[2] + 83;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 349u; }
    cells[0] = cells[1] + 617;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }