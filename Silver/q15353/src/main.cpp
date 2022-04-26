#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

string bigint_sum(string a, string b);

int main(void){

    ios_base :: sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    string a = "";
    string b = "";
    cin >> a >> b;

    string result = bigint_sum(a,b);
    cout << result;
}

string bigint_sum(string a, string b){

    string output;
    vector<int> result;
    int carry = 0;

    reverse(a.begin(), a.end());
    reverse(b.begin(), b.end());

    if (a.length() > b.length()) {
        b.resize((a.length()));
    } else if (a.length() < b.length()) {
        a.resize((b.length()));
    } result.resize(a.length());

    for (int i = 0; i < result.size(); i++){
        if (a[i] == '\0') a[i] = '0';
        if (b[i] == '\0') b[i] = '0';
    }

    for (int i = 0; i < result.size(); i++){
        result[i] = (a[i] - '0' + b[i] - '0' + carry) % 10;
        if ((a[i] - '0' + b[i] - '0' + carry) / 10 == 1) {
            carry = (a[i] - '0' + b[i] - '0' + carry) / 10;
        } else { carry = 0; }
    } if (carry == 1) { result.push_back(carry); }

    while (!result.back()) {
        result.pop_back();
    } reverse(result.begin(), result.end());
    output.resize(result.size());

    for (int i = 0; i < result.size(); i++){
        output[i] = result[i] + '0';
    } output.append("\0");

    return output;
}