class LinkedList {

private:
    class Node {

    public:
        int value;
        Node* next;
        Node(int value) : value(value), next(nullptr) {}
    };

    Node* head;
    Node* tail;
    int length;

public:
    LinkedList(int value) {
        Node* newNode = new Node(value);
        head = newNode;
        tail = newNode;
        length = 1;
    }
};

int main() { LinkedList* LinkedListOne = new LinkedList(4); }
