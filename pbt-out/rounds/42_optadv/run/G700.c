static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 53; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[0 % 6] = 813; cells[7] += arr[0 % 6];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    cells[0] = cells[4] + 207;
    cells[2] = cells[7] + 130;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }