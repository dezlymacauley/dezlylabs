#include <print>

using std::println;

class LinkedList {

private:
    class Node {

    public:
        int value;
        Node* next;
        Node(int value) : value(value), next(nullptr) {
        }
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

    void printList() {
        Node* temp = head;

        while (temp != nullptr) {
            println("{}", temp->value);
            temp = temp->next;
        }
    }

    void printFirstValue() {
        println("The value of the first element is: {}", head->value);
    }
    
    void printLastValue() {
        println("The value of the last element is: {}", tail->value);
    }
    
    void printLength() {
        println("Number of elements in this linked list: {}", this->length);
    }

};


int main() {
    LinkedList* linkedListOne = new LinkedList(4);
    linkedListOne->printList();
    // 4

    linkedListOne->printFirstValue();
    // The value of the first element is: 4

    linkedListOne->printLastValue();
    // The value of the last element is: 4

    linkedListOne->printLength();
    // Number of elements in this linked list: 1

}
