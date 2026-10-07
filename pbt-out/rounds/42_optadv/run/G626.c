static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 882u; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[2] = cells[0] + 636;
    cells[4] = cells[0] + 776;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[0] = cells[6] + 503;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }