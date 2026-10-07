static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 115; cells[0] += arr[1 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[5 % 6] = 400; cells[5] += arr[5 % 6];
    cells[4] = cells[1] + 885;
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    arr[0 % 6] = 77; cells[4] += arr[0 % 6];
    arr[0 % 6] = 34; cells[2] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }